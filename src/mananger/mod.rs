mod download;
mod manage;
pub mod messages;
mod watch_changer;

use std::sync::Arc;

use crate::errors::error::Result;
use indicatif::{MultiProgress, ProgressBar};
use lock::Lock;
use log::warn;
use manage::manage;
use settings::Settings;
use tokio::{
    sync::{mpsc, Mutex, RwLock},
    try_join,
};
use tr::load::Load;

use crate::{lock, settings, tr};

use self::watch_changer::watch_changes;

pub async fn run() -> Result<()> {
    let (mpb, lock, settings) = init().await?;
    //
    let pb = mpb.add(ProgressBar::new_spinner());
    pb.finish_with_message("Init Minecraft Addon Controller");
    //
    let (tx, rx) = mpsc::channel(20);

    let manage = {
        let settings_m = Arc::clone(&settings);
        let lock = Arc::clone(&lock);
        let mpb_m = Arc::clone(&mpb);
        manage(rx, lock, settings_m, mpb_m)
    };
    let watch_changes = {
        let settings_w = Arc::clone(&settings);
        let lock = Arc::clone(&lock);
        let mpb_w = Arc::clone(&mpb);
        watch_changes(settings_w, lock, mpb_w, tx)
    };

    let a = try_join!(manage, watch_changes);
    match a {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

async fn init() -> Result<(Arc<MultiProgress>, Arc<Mutex<Lock>>, Arc<RwLock<Settings>>)> {
    let mpb: Arc<MultiProgress> = Arc::new(MultiProgress::new());
    let lock: Arc<Mutex<Lock>> = Arc::new(Mutex::new(Lock::load().await.unwrap_or({
        warn!("Use default Lock");
        Lock::default()
    })));
    let settings = Arc::new(RwLock::new(Settings::load().await?));
    Ok((mpb, lock, settings))
}
