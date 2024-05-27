use crate::DICTIONARY;
use std::sync::Arc;

use indicatif::MultiProgress;
use tokio::sync::{mpsc::Receiver, Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use crate::errors::error::Result;
use crate::manager::download::download;
use crate::manager::messages::Messages;
use crate::{lock::Lock, settings::Settings};

pub async fn manage(
    mut rx: Receiver<Messages>,
    lock: Arc<Mutex<Lock>>,
    settings: Arc<RwLock<Settings>>,
    mpb: Arc<MultiProgress>,
) -> Result<()> {
    let lock = Arc::clone(&lock);
    let settings = Arc::clone(&settings);
    let mpb = Arc::clone(&mpb);
    let key = Arc::new(CancellationToken::new());
    loop {
        tokio::select! {
            Some(e) = rx.recv() => match e {
                Messages::Restart(pb) => {
                    pb.set_message(DICTIONARY.manager().restart());
                    let key = Arc::clone(&key);
                    pb.set_message(DICTIONARY.manager().stop_iteration());
                    key.cancel();
                    mpb.clear()?;
                    if key.is_cancelled() {
                        pb.set_message(DICTIONARY.manager().start_new_iteration());
                        tokio::spawn(download(settings.clone(), lock.clone(), mpb.clone(), key));
                    } else {
                        pb.set_message(DICTIONARY.manager().waiting_new_iteration());
                    }
                    pb.finish_and_clear();
                }
                Messages::Stop(pb) => {
                    pb.finish_with_message(DICTIONARY.manager().stop_iteration());
                    key.cancelled().await;
                }
                Messages::Start(pb) => {
                    let key = Arc::clone(&key);
                    pb.finish_with_message(DICTIONARY.manager().start_new_iteration());
                    tokio::spawn(download(settings.clone(), lock.clone(), mpb.clone(), key));
                    pb.finish_and_clear();
                }
            },
        }
    }
}
