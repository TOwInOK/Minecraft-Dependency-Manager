pub mod plugins;
mod versions;
pub mod core;
mod additions;
mod lock;

use crate::errors::errors::ConfigErrors;
use additions::Additions;
use std::collections::HashMap;
use core::Core;
use versions::Versions;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use tokio::fs;
use plugins::Plugin;
/// Структура для инициализации конфигурации
///
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    /// Minecraft core
    #[serde(default)]
    pub core: Core,
    /// Лист плагинов
    /// [name]:[Plugin] 
    #[serde(default)]
    pub plugins: HashMap<String, Plugin>,
    /// Additions for git or keys
    #[serde(default)]
    pub additions: Option<Additions>,
}

impl Config {
    pub async fn load_config(path: String) -> Result<Config, ConfigErrors> {
        info!("Загрузка конфигурационного файла...");
        let toml = fs::read_to_string(&path).await?;
        info!("Файл успешно загружен.");

        info!("Инициализация конфигурационного файла...");
        let config: Config = toml::from_str(&toml)?;
        info!("Конфигурация успешно инициализирована.");

        Ok(config)
    }
} 