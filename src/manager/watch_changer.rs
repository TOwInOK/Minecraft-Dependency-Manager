use std::sync::Arc;
use std::time::Duration;

use async_watcher::notify::RecursiveMode;
use async_watcher::AsyncDebouncer;
use indicatif::ProgressBar;
use log::debug;
use tokio::sync::mpsc::Sender;

use crate::errors::error::Result;
use crate::manager::messages::Messages;
use crate::settings::Settings;
use crate::tr::load::Load;
use crate::{DICTIONARY, LOCK, MPB, SETTINGS};

/// Load downloader module.
/// Always check config file.
/// Use `token` for canceling minecraft task
pub async fn watch_changes(manager_tx: Sender<Messages>) -> Result<()> {
    const CONFIG_PATH: &str = "settings.toml";
    // initialize the debouncer
    let (mut tx, mut rx) = AsyncDebouncer::new_with_channel(Duration::from_millis(200), None)
        .await
        .unwrap();
    // register path to watch
    tx.watcher()
        .watch(CONFIG_PATH.as_ref(), RecursiveMode::NonRecursive)
        .unwrap();

    let pb = Arc::new(MPB.add(ProgressBar::new_spinner()));
    // Check lock
    {
        debug!("Start remove_nonexistent");
        LOCK.lock().await.remove_defunct(pb.clone()).await;
    }
    // Send start to downloader
    {
        debug!("Start downloader (message)");
        manager_tx.send(Messages::Start(pb.clone())).await?;
    }

    // wait for events
    while rx.recv().await.is_some() {
        debug!("find iteration");
        let pb = Arc::clone(&pb);
        pb.clone()
            .set_message(DICTIONARY.config().find_changes_in_settings());
        let settings_new = Settings::load().await?;
        if *SETTINGS.read().await != settings_new {
            pb.set_message(DICTIONARY.config().settings_changed());
            *SETTINGS.write().await = settings_new;
            {
                debug!("Start remove_nonexistent");
                let pb = Arc::clone(&pb);
                LOCK.lock().await.remove_defunct(pb).await;
            }
            debug!("Start downloader (message)");
            pb.set_message(DICTIONARY.config().settings_rewritten());
            manager_tx.send(Messages::Restart(pb)).await?;
        } else {
            debug!("Nothing to update (config)");
            pb.set_message(DICTIONARY.config().settings_same());
            pb.finish_and_clear();
        }
    }
    Ok(())
}
