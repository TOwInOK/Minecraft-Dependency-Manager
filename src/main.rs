mod config;
mod controller;
mod downloader;
mod errors;
mod lock;


#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    
    controller::Controller::init().await.watch_config_changes().await;
}
