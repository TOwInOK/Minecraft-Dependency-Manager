mod config;
mod controller;
mod downloader;
mod errors;
mod lock;

use std::time::Duration;

use config::Config;
use downloader::Downloader;
use log::error;
use tokio::{task, time};

use crate::lock::lock::Lock;

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

    //Load lock
    let mut lock = Lock::new();
    if let Err(e) = lock.load(&config.additions.path_to_configs).await {
        error!("{e}");
        lock.create(&config.additions.path_to_configs)
            .await
            .unwrap();
        lock.load(&config.additions.path_to_configs).await.unwrap();
    }

    // Запускаем проверку.
    let downloader_checker = task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(300));

        loop {
            interval.tick().await;
            Downloader::new(&mut config, &mut lock)
                .check()
                .await
                .unwrap_or_else(|e| error!("{e}"));
        }
    });

    // Ожидаем завершения обеих задач
    let _ = tokio::try_join!(downloader_checker);
}
