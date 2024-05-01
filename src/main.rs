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
use settings::Settings;
use tokio::sync::Mutex;
use tr::load::Load;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let mpb = Arc::new(Mutex::new(MultiProgress::new()));
    let pb = mpb.lock().await.add(ProgressBar::new_spinner());
    pb.set_message("Init Minecraft Addon Controller");
    let lock = Arc::new(Mutex::new(Lock::load().await.unwrap_or_default()));
    let settings = Settings::load().await?;
    // let a = Additions::new(Some("GitHub.link".to_string()), Some("GitHub.key".to_string()));
    // settings.set_additions(Some(a));
    lock.lock().await.remove_nonexistent(&settings)?;
    if let Some(plugins) = settings.plugins() {
        plugins
            .download_all(settings.core().version(), &lock, &mpb)
            .await?;
    }
    settings.core().download(&lock, &mpb).await?;
    Ok(())
}
