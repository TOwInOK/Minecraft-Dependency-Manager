mod download;
mod manage;
pub mod messages;
mod watch_changer;

use std::sync::Arc;
use std::time::Duration;

use self::watch_changer::watch_changes;
use crate::errors::error::Result;
use crate::settings::extensions::plugin::Plugin;
use crate::tr::save::Save;
use crate::{lock, settings, tr, DICTIONARY, MPB};
use indicatif::ProgressBar;
use indicatif_log_bridge::LogWrapper;
use lock::Lock;
use log::warn;
use manage::manage;
use settings::Settings;

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

    '_plugin_folder_scope: {
        use tokio::fs::create_dir;
        use tokio::fs::read_dir;

        if read_dir(Plugin::PATH).await.is_err() {
            create_dir(Plugin::PATH).await?
        }
    }

    LogWrapper::new(MPB.as_ref().clone(), logger)
        .try_init()
        .unwrap();
    // Init It!
    let pb = MPB.add(ProgressBar::new_spinner());
    pb.set_message(DICTIONARY.intro());
    sleep(Duration::from_secs(1)).await;
    pb.finish_and_clear();
    //
    let (tx, rx) = mpsc::channel(20);

    let manage = manage(rx);
    let watch_changes = watch_changes(tx);

    let a = try_join!(manage, watch_changes);
    match a {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn load_settings() -> Result<Arc<RwLock<Settings>>> {
    use std::fs::File;
    use std::io::Write;
    '_default_settings_scope: {
        let path = <Settings as Load>::PATH;
        if File::open(path).is_err() {
            let default = Settings::default();
            warn!("Create default config file");
            let mut file = File::create(path)?;
            let toml_default = toml::to_string_pretty(&default)?;
            file.write_all(toml_default.as_bytes())?;
        }
    }
    let settings = Arc::new(RwLock::new(Settings::load_sync()?));
    Ok(settings)
}

pub fn load_lock() -> Result<Arc<Mutex<Lock>>> {
    use std::fs::File;
    use std::io::Write;
    '_default_lock_scope: {
        let path = <Lock as Load>::PATH;
        if File::open(path).is_err() {
            let default = Lock::default();
            warn!("Create default Lock file");
            let mut file = File::create(path)?;
            let toml_default = toml::to_string_pretty(&default)?;
            file.write_all(toml_default.as_bytes())?;
        }
    }
    let lock: Arc<Mutex<Lock>> = Arc::new(Mutex::new(Lock::load_sync()?));
    Ok(lock)
}

pub async fn load_dictionary() {}
