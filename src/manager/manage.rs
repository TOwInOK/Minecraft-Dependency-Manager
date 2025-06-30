use crate::{DICTIONARY, MPB};
use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::errors::error::Result;
use crate::manager::download::download;
use crate::manager::messages::Messages;

pub async fn manage(mut rx: Receiver<Messages>) -> Result<()> {
    let key = Arc::new(CancellationToken::new());
    loop {
        tokio::select! {
            Some(e) = rx.recv() => match e {
                Messages::Restart(pb) => {
                    pb.set_message(DICTIONARY.manager().restart());
                    let key = Arc::clone(&key);
                    pb.set_message(DICTIONARY.manager().stop_iteration());
                    key.cancel();
                    MPB.clear()?;
                    if key.is_cancelled() {
                        pb.set_message(DICTIONARY.manager().start_new_iteration());
                        tokio::spawn(download(key));
                    } else {
                        pb.set_message(DICTIONARY.manager().waiting_new_iteration());
                    }
                }
                Messages::Stop(pb) => {
                    pb.finish_with_message(DICTIONARY.manager().stop_iteration());
                    key.cancelled().await;
                }
                Messages::Start(pb) => {
                    let key = Arc::clone(&key);
                    pb.finish_with_message(DICTIONARY.manager().start_new_iteration());
                    tokio::spawn(download(key));
                }
            },
        }
    }
}
