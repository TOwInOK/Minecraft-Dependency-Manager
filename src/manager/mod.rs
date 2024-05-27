mod download;
mod manage;
pub mod messages;
mod watch_changer;

use std::sync::Arc;
use std::time::Duration;

use self::watch_changer::watch_changes;
use crate::dictionary::dictionary::MessageDictionary;
use crate::errors::error::Result;
use crate::settings::extensions::plugin::Plugin;
use crate::tr::save::Save;
use crate::{lock, settings, tr, DICTIONARY};
use indicatif::{MultiProgress, ProgressBar};
use indicatif_log_bridge::LogWrapper;
use lock::Lock;
use log::warn;
use manage::manage;
use settings::Settings;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tokio::{
    sync::{mpsc, Mutex, RwLock},
    try_join,
};
use tr::load::Load;

pub async fn run() -> Result<()> {
    let logger = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .build();
    let (mpb, lock, settings) = init().await?;
    let mpb_cloned = mpb.as_ref().clone();
    LogWrapper::new(mpb_cloned, logger).try_init().unwrap();
    // Init It!
    let pb = mpb.add(ProgressBar::new_spinner());
    pb.set_message(DICTIONARY.intro());
    sleep(Duration::from_secs(1)).await;
    pb.finish_and_clear();
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
    '_plugin_folder_scope: {
        if fs::read_dir(Plugin::PATH).await.is_err() {
            fs::create_dir(Plugin::PATH).await?
        }
    }
    '_default_settings_scope: {
        let path = <Settings as Load>::PATH;
        if File::open(path).await.is_err() {
            let default = Settings::default();
            warn!("Create default config file");
            let mut file = File::create(path).await?;
            let toml_default = toml::to_string_pretty(&default)?;
            file.write_all(toml_default.as_bytes()).await?;
        }
    }
    '_default_lock_scope: {
        let path = <Lock as Load>::PATH;
        if File::open(path).await.is_err() {
            let default = Lock::default();
            warn!("Create default Lock file");
            let mut file = File::create(path).await?;
            let toml_default = toml::to_string_pretty(&default)?;
            file.write_all(toml_default.as_bytes()).await?;
        }
    }
    '_default_language_scope: {
        let path = <MessageDictionary as Load>::PATH;
        if File::open(path).await.is_err() {
            let default = MessageDictionary::default();
            warn!("Create default language file");
            let mut file = File::create(path).await?;
            let toml_default = toml::to_string_pretty(&default)?;
            file.write_all(toml_default.as_bytes()).await?;
        }
    }
    let mpb: Arc<MultiProgress> = Arc::new(MultiProgress::new());
    let lock: Arc<Mutex<Lock>> = Arc::new(Mutex::new(Lock::load().await?));
    let settings = Arc::new(RwLock::new(Settings::load().await?));
    Ok((mpb, lock, settings))
}
