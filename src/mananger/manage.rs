use std::sync::Arc;

use indicatif::MultiProgress;
use tokio::sync::{mpsc::Receiver, Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use crate::errors::error::Result;
use crate::mananger::download::download;
use crate::mananger::messages::Messages;
use crate::{lock::Lock, settings::Settings};

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
                    pb.set_message("Рестарт!");
                    let key = Arc::clone(&key);
                    pb.set_message("Стопаем текущюю задачу!");
                    key.cancel();
                    mpb.clear()?;
                    if key.is_cancelled() {
                        pb.set_message("Начинаем новую!");
                        tokio::spawn(download(settings.clone(), lock.clone(), mpb.clone(), key));
                    } else {
                        pb.set_message("Ну... она стоит");
                    }
                    pb.finish_and_clear();
                }
                Messages::Stop(pb) => {
                    pb.finish_with_message("Остановка!");
                    key.cancelled().await;
                }
                Messages::Start(pb) => {
                    let key = Arc::clone(&key);
                    pb.finish_with_message("Начинаем новую!");
                    tokio::spawn(download(settings.clone(), lock.clone(), mpb.clone(), key));
                    pb.finish_and_clear();
                }
            },
        }
    }
}
