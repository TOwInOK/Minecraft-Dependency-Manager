use crate::tr::load::Load;
use std::sync::Arc;

use indicatif::MultiProgress;
use tokio::sync::{mpsc::Receiver, Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use crate::dictionary::pb_messages::PbMessages;
use crate::errors::error::Result;
use crate::mananger::download::download;
use crate::mananger::messages::Messages;
use crate::{lock::Lock, settings::Settings};
use lazy_static::lazy_static;

lazy_static! {
    static ref DICT: PbMessages = PbMessages::load_sync().unwrap();
}

pub async fn manage(
    mut rx: Receiver<Messages>,
    lock: Arc<Mutex<Lock>>,
    settings: Arc<RwLock<Settings>>,
    mpb: Arc<MultiProgress>,
) -> Result<()> {
    loop {
        let lock = Arc::clone(&lock);
        let settings = Arc::clone(&settings);
        let mpb = Arc::clone(&mpb);
        let key = Arc::new(CancellationToken::new());
        tokio::select! {
            Some(e) = rx.recv() => match e {
                Messages::Restart(pb) => {
                    pb.set_message(&DICT.restart);
                    let key = Arc::clone(&key);
                    pb.set_message(&DICT.stop_iteration);
                    key.cancel();
                    mpb.clear()?;
                    if key.is_cancelled() {
                        pb.set_message(&DICT.start_new_iteration);
                        tokio::spawn(download(settings.clone(), lock.clone(), mpb.clone(), key));
                    } else {
                        pb.set_message(&DICT.waiting_new_iteration);
                    }
                    pb.finish_and_clear();
                }
                Messages::Stop(pb) => {
                    pb.finish_with_message(&DICT.stop_iteration);
                    key.cancelled().await;
                }
                Messages::Start(pb) => {
                    let key = Arc::clone(&key);
                    pb.finish_with_message(&DICT.start_new_iteration);
                    tokio::spawn(download(settings.clone(), lock.clone(), mpb.clone(), key));
                    pb.finish_and_clear();
                }
            },
        }
    }
}
