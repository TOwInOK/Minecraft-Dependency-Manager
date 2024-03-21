mod config;
mod downloader;
mod errors;

use config::Config;
use downloader::Downloader;
use log::error;
#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    //Load Config file
    let path = "./config.toml".to_string();
    let mut config = Config::load_config(path).await.unwrap_or_else(|e| {
        log::error!("message: {}", e);
        log::warn!("Происходит загрузка стандартного конфига");
        Config::default()
    });
    log::debug!("{:#?}", config);
    let downloader = Downloader::new().await;
    downloader
        .check(&mut config)
        .await
        .unwrap_or_else(|e| error!("{e}"));
}
