pub mod dictionary;
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
    mananger::run().await
}
