use std::sync::Arc;
use std::time::Duration;

use async_watcher::notify::RecursiveMode;
use async_watcher::AsyncDebouncer;
use indicatif::{MultiProgress, ProgressBar};
use log::debug;
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, RwLock};

use crate::errors::error::Result;
use crate::mananger::messages::Messages;
use crate::tr::load::Load;
use crate::{lock::Lock, settings::Settings};

use crate::dictionary::pb_messages::PbMessages;
use lazy_static::lazy_static;

lazy_static! {
    static ref DICT: PbMessages = PbMessages::load_sync().unwrap();
}
/// Load downloader module.
/// Always check config file.
/// Use `token` for canceling minecraft task
pub async fn watch_changes(
    settings: Arc<RwLock<Settings>>,
    lock: Arc<Mutex<Lock>>,
    mpb: Arc<MultiProgress>,
    manager_tx: Sender<Messages>,
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

    let pb = Arc::new(mpb.add(ProgressBar::new_spinner()));
    // Check lock
    {
        debug!("Start remove_nonexistent");
        let pb = Arc::clone(&pb);
        let settings = Arc::clone(&settings);
        lock.lock().await.remove_nonexistent(settings, pb).await;
    }
    // Send start to downloader
    {
        debug!("Start downloader (message)");
        let pb = Arc::clone(&pb);
        manager_tx.send(Messages::Start(pb)).await?;
    }

    // wait for events
    while rx.recv().await.is_some() {
        debug!("find iteration");
        let pb = Arc::clone(&pb);
        pb.set_message(&DICT.find_changes_in_settings);
        let settings_new = Settings::load().await?;
        let settings = Arc::clone(&settings);
        if *settings.read().await != settings_new {
            pb.set_message(&DICT.settings_changed);
            *settings.write().await = settings_new;
            {
                debug!("Start remove_nonexistent");
                let pb = Arc::clone(&pb);
                lock.lock().await.remove_nonexistent(settings, pb).await;
            }
            debug!("Start downloader (message)");
            pb.set_message(&DICT.settings_has_rewrited);
            manager_tx.send(Messages::Restart(pb)).await?;
        } else {
            debug!("Nothing to update (config)");
            pb.set_message(&DICT.settings_same);
            pb.finish_and_clear();
        }
    }
    Ok(())
}
