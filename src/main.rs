pub mod errors;
pub mod lock;
pub mod models;
pub mod query;
pub mod settings;
pub mod tr;

use crate::errors::error::Result;
use lock::Lock;
use settings::Settings;
use tr::load::Load;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let mut lock = Lock::load().await?;
    let settings = Settings::load().await?;
    // let a = Additions::new(Some("GitHub.link".to_string()), Some("GitHub.key".to_string()));
    // settings.set_additions(Some(a));

    if let Some(plugins) = settings.plugins() {
        plugins
            .download_all(settings.core().version(), &mut lock)
            .await?;
    }
    settings.core().download(&mut lock).await?;
    Ok(())
}
