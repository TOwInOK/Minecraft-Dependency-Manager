pub mod dictionary;
pub mod errors;
pub mod lock;
pub mod manager;
pub mod models;
pub mod query;
pub mod settings;
pub mod tr;

use crate::errors::error::Result;
use dictionary::dictionary::MessageDictionary;
use once_cell::sync::Lazy;

static DICTIONARY: Lazy<MessageDictionary> = Lazy::new(|| MessageDictionary::get_dict());
#[tokio::main]
async fn main() -> Result<()> {
    manager::run().await
}
