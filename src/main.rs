pub mod errors;
pub mod lock;
pub mod models;
pub mod query;
pub mod settings;
pub mod tr;

use std::sync::Arc;

use crate::errors::error::Result;
use indicatif::{MultiProgress, ProgressBar};
use lock::Lock;
use log::warn;
use settings::Settings;
use tokio::sync::Mutex;
use tr::load::Load;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    // Init
    let (mpb, lock, settings) = init().await;
    //
    let pb = mpb.add(ProgressBar::new_spinner());
    pb.finish_with_message("Init Minecraft Addon Controller");
    //
    lock.lock().await.remove_nonexistent(&settings)?;
    '_c: {
        let lock = Arc::clone(&lock);
        let settings = Arc::clone(&settings);
        let mpb = Arc::clone(&mpb);
        download(settings, lock, mpb).await?;
    }
    Ok(())
}

async fn download(
    settings: Arc<Settings>,
    lock: Arc<Mutex<Lock>>,
    mpb: Arc<MultiProgress>,
) -> Result<()> {
    '_core_scope: {
        let lock = Arc::clone(&lock);
        let mpb = Arc::clone(&mpb);
        settings.core().download(lock, mpb).await?;
    }
    '_plugins_scope: {
        let lock = Arc::clone(&lock);
        let mpb = Arc::clone(&mpb);
        if let Some(plugins) = settings.plugins() {
            plugins
                .download_all(
                    settings.core().provider().as_str(),
                    settings.core().version(),
                    lock,
                    mpb,
                )
                .await?;
        }
    }
    Ok(())
}

async fn init() -> (Arc<MultiProgress>, Arc<Mutex<Lock>>, Arc<Settings>) {
    let mpb: Arc<MultiProgress> = Arc::new(MultiProgress::new());
    let lock: Arc<Mutex<Lock>> = Arc::new(Mutex::new(Lock::load().await.unwrap_or({
        warn!("Use default Lock");
        Lock::default()
    })));
    let settings: Arc<Settings> = Arc::new(Settings::load().await.unwrap());
    (mpb, lock, settings)
}
