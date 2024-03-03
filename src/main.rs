mod config;
use config::Config;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    //Load Config file
    let path = "./config.toml".to_string();
    let config = Config::load_config(path).await.unwrap_or_else(|e| {
        log::error!("message: {}", e);
        log::warn!("Происходит загрузка стандартного конфига");
        Config::default()
    });
    log::debug!("{:#?}", config);
    match config.download_all().await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
