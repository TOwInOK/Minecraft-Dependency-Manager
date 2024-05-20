use std::sync::Arc;
use std::time::Duration;

use async_watcher::notify::RecursiveMode;
use async_watcher::AsyncDebouncer;
use indicatif::{MultiProgress, ProgressBar};
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, RwLock};

use crate::errors::error::Result;
use crate::mananger::messages::Messages;
use crate::tr::load::Load;
use crate::{lock::Lock, settings::Settings};

/// Load downloader module.
/// Always check config file.
/// Use `token` for canceling minecraft task
pub async fn watch_changes(
    settings: Arc<RwLock<Settings>>,
    lock: Arc<Mutex<Lock>>,
    mpb: Arc<MultiProgress>,
    dw_tx: Sender<Messages>,
) -> Result<()> {
    const CONFIG_PATH: &str = "settings.toml";
    // initialize the debouncer
    let (mut tx, mut rx) = AsyncDebouncer::new_with_channel(Duration::from_millis(200), None)
        .await
        .unwrap();
    // register path to watch
    tx.watcher()
        .watch(CONFIG_PATH.as_ref(), RecursiveMode::NonRecursive)
        .unwrap();

    // Check lock
    {
        lock.lock()
            .await
            .remove_nonexistent(settings.read().await)?;
    }
    let pb = Arc::new(mpb.add(ProgressBar::new_spinner()));
    // Send start to downloader
    {
        dw_tx.send(Messages::Start(Arc::clone(&pb))).await?;
    }

    // wait for events
    while rx.recv().await.is_some() {
        let pb = Arc::clone(&pb);
        pb.set_message("Find some changes in config!");
        let settings = Arc::clone(&settings);
        let settings_new = Settings::load().await?;
        if *settings.read().await != settings_new {
            pb.set_message("настройки другие");
            *settings.write().await = settings_new;
            {
                lock.lock()
                    .await
                    .remove_nonexistent(settings.read().await)?;
            }
            pb.set_message("настройки перезаписали");
            dw_tx.send(Messages::Restart(pb)).await?;
        } else {
            pb.set_message("настройки теже");
        }
    }
    Ok(())
}
