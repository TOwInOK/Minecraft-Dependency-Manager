pub mod additions;
pub mod core;
pub mod plugins;
pub mod versions;

use crate::errors::error::ConfigErrors;
use additions::Additions;
use core::Core;
use log::info;
use plugins::Plugin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use versions::Versions;
/// Структура для инициализации конфигурации
///
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    /// Minecraft core
    #[serde(default)]
    pub core: Core,
    /// Plugin list
    /// [name]:[Plugin]
    #[serde(default)]
    pub plugins: HashMap<String, Plugin>,
    /// Additions for git or keys
    #[serde(default)]
    pub additions: Additions,
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
