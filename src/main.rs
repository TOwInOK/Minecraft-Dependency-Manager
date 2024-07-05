pub mod dictionary;
pub mod errors;
pub mod lock;
pub mod manager;
pub mod models;
pub mod query;
pub mod settings;
pub mod tr;

use std::sync::Arc;

use crate::errors::error::Result;
use dictionary::MessageDictionary;
use indicatif::MultiProgress;
use lock::Lock;
use manager::{load_lock, load_settings};
use once_cell::sync::Lazy;
use settings::Settings;
use tokio::sync::{Mutex, RwLock};

static DICTIONARY: Lazy<MessageDictionary> = Lazy::new(|| MessageDictionary::get_dict().unwrap());
static MPB: Lazy<Arc<MultiProgress>> = Lazy::new(|| Arc::new(MultiProgress::new()));
static SETTINGS: Lazy<Arc<RwLock<Settings>>> = Lazy::new(|| load_settings().unwrap());
static LOCK: Lazy<Arc<Mutex<Lock>>> = Lazy::new(|| load_lock().unwrap());

#[tokio::main]
async fn main() -> Result<()> {
    manager::run().await
}
