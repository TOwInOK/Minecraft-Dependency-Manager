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
    pretty_env_logger::init();
    // let mut query = Query::default();

    // for (name, plugin) in settings.plugins().items().iter() {
    //     let game_version = settings.core().version();
    //     let a = plugin.get_link(name, game_version).await?;
    //     query.query_mut().insert(name.to_string(), a.into());
    // }
    // let settings = Arc::new(Mutex::new(Settings::default()));
    // let lock = Arc::new(Mutex::new(Lock::default()));
    // let plugins = Plugins::new(path, items);
    // settings.get_mut().plugins_mut()
    // lock.lock().await.save().await?;
    // settings.lock().await.save().await?;

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
