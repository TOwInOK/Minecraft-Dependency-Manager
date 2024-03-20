mod downloader;
mod errors;
mod plugins;
mod versions;
mod core;
mod additions;

use additions::Additions;
use std::collections::HashMap;
use core::Core;
use versions::Versions;
use errors::*;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use tokio::fs;
use plugins::Plugin;
/// Структура для инициализации конфигурации
///
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    /// Minecraft core
    #[serde(default)]
    core: Core,
    /// Лист плагинов
    /// [name]:[Plugin] 
    #[serde(default)]
    plugins: HashMap<String, Plugin>,
    /// Additions for git or keys
    additions: Option<Additions>,
}

impl Default for Config {
    fn default() -> Self {
        warn!("Не обнаружен конфигурационный файл!\nЗагрузка стандартной конфигурации!");
        Self { core: Default::default(), plugins: Default::default(), additions: None }
    }
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

    // pub async fn download_all(self) -> Result<(), DownloadErrors> {
    //     //download core
    //         self.choose_core().await?;
    //         self.choose_plugin().await
    // }

    // ///Function download core by info in [`Config`]
    // async fn choose_core(&self) -> Result<(), DownloadErrors> {
    //     match &self.version {
    //         //Download vanilla
    //         Versions::Vanilla(ver, freeze) => {
    //             let (link, hash) = Vanilla::find(&**ver).await?;
    //             Downloader::download_core(*freeze, link, hash).await
    //         }
    //         Versions::Purpur(_, _) => todo!(),
    //         Versions::Paper(_, _) => todo!(),
    //         Versions::Spigot(_, _) => todo!(),
    //         Versions::Bucket(_, _) => todo!(),
    //     }
    // }
    // async fn choose_plugin(&self) -> Result<(), DownloadErrors> {
    //     if let Some(plugins) = &self.plugins {
            
    //         todo!()
    //     } else {
    //         info!("Нет плагинов для скачивания");
    //         Ok(())
    //     }
    // }
} 