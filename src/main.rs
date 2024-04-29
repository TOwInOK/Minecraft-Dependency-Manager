pub mod errors;
pub mod lock;
pub mod models;
pub mod query;
pub mod settings;
pub mod tr;

use crate::errors::error::Result;
use crate::lock::Lock;
use settings::Settings;
use std::sync::Arc;
use tokio::sync::Mutex;
use tr::{load::Load, save::Save};

#[tokio::main]
async fn main() -> Result<()> {
    // let lock = Lock::default();
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

    let load_settings = Settings::load().await?;
    println!("{:#?}", load_settings);
    load_settings.save().await?;
    Ok(())
}
