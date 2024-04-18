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

    let _ = controller::Controller::init().await;
}
