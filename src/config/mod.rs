mod datapack;
mod errors;
mod model;
mod plugin;
mod version;

use datapack::*;
use errors::*;
use log::{error, info};
use plugin::Plugin;
use serde::{Deserialize, Serialize};
use std::default;
use tokio::fs;
use version::Versions;

///Struct to load config from toml file.
#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    version: Versions,
    plugins: Option<Plugin>,
    datapacks: Option<Datapack>,
}

impl Config {
    fn new(version: Versions, plugins: Option<Plugin>, datapacks: Option<Datapack>) -> Self {
        Self {
            version,
            plugins,
            datapacks,
        }
    }

    fn default() -> Self {
        Config::new(Versions::default(), None, None)
    }

    pub async fn load_config(path: String) -> Config {
        let toml = {
            info!("Загрузка конфигурационного файла...");
            let result = fs::read_to_string(path).await;
            match result {
                Ok(content) => {
                    info!("Файл успешно загружен.");
                    content
                }
                Err(e) => {
                    error!(
                        "Ваш конфигурационный файл не был обнаружен, загружаю стандартные настройки.\nПричина ошибки: {e}"
                    );
                    return Config::default();
                }
            }
        };
        info!("Инициализация конфигурационного файла...");
        let config: Config = match toml::from_str(&toml) {
            Ok(parsed_config) => {
                info!("Конфигурация успешно инициализированна.");
                parsed_config
            }
            Err(e) => {
                error!("Не удалось загрузить конфигурацию, загружаю стандартные настройки.\nПричина ошибки: {e}");
                return Config::default();
            }
        };
        config
    }

    pub async fn download(config: Config) -> Result<(), DownloadErrors> {
        let file = config.download_core().await;
        todo!()
    }

    async fn download_plugins() -> Result<(), DownloadErrors> {
        todo!()
    }
    async fn download_mods() -> Result<(), DownloadErrors> {
        todo!()
    }
    async fn download_datapacks() -> Result<(), DownloadErrors> {
        todo!()
    }

    async fn download_core(self) -> Result<Option<()>, DownloadErrors> {
        match self.version {
            Versions::Purpur(ver, freez) => {
                if !freez {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                Err(DownloadErrors::DownloadCorrapt("ff".to_string()))
            }
            Versions::Paper(ver, freez) => todo!(),
            Versions::Spigot(ver, freez) => todo!(),
            Versions::Bucket(ver, freez) => todo!(),
            Versions::Vanila(ver, freez) => todo!(),
        }
    }
}
