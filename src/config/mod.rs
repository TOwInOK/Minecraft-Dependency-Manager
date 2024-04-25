pub mod additions;
pub mod core;
mod models;
pub mod plugins;

use crate::{downloader::hash::ChooseHash, errors::error::Result};
use additions::Additions;
use core::Core;
use log::info;
use plugins::Plugin;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
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
    pub async fn load_config(path: &str) -> Result<Config> {
        info!("Загрузка конфигурационного файла...");
        let toml = fs::read_to_string(&path).await?;
        info!("Файл успешно загружен.");

        info!("Инициализация конфигурационного файла...");
        let config: Config = toml::from_str(&toml)?;
        info!("Конфигурация успешно инициализирована.");

        Ok(config)
    }
    pub async fn get_core_link(self) -> Result<(String, ChooseHash, String)> {
        self.core.get_link().await
    }
}
