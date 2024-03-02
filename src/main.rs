mod config;
use config::Config;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder().filter_level(log::LevelFilter::Debug).init();

    //Load Config file
    let path = "./config.toml".to_string();
    let config = Config::load_config(path).await;
    log::info!("Config: {:#?}", config);
    match Config::download(config).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }   
}
