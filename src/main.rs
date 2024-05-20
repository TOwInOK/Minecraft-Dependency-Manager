pub mod errors;
pub mod lock;
pub mod mananger;
pub mod models;
pub mod query;
pub mod settings;
pub mod tr;

use crate::errors::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    mananger::run().await
}
