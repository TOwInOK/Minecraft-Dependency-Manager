mod datapack;
mod errors;
mod plugin;
mod version;
mod models;

use models::{vanila::Vanila, *};
use tempfile::Builder;
use datapack::*;
use errors::*;
use log::{error, info};
use plugin::Plugin;
use serde::{Deserialize, Serialize};
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

    ///Function download core by info in [`Config`] 
    async fn download_core(self) -> Result<Option<()>, DownloadErrors> {
        match self.version {
            //Download purpur
            Versions::Purpur(ver, freez) => {
                if freez {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                Err(DownloadErrors::DownloadCorrapt("ff".to_string()))
            }
            //Download paper
            Versions::Paper(ver, freez) => {
                if freez {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                Err(DownloadErrors::DownloadCorrapt("ff".to_string()))
            },
            //Download Spigot
            Versions::Spigot(ver, freez) => {
                if freez {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                Err(DownloadErrors::DownloadCorrapt("ff".to_string()))
            },
            //Download Bucket
            Versions::Bucket(ver, freez) => {
                if freez {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                Err(DownloadErrors::DownloadCorrapt("ff".to_string()))
            },
            //Download Vanila
            Versions::Vanila(ver, freez) => {
                if freez {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                // Err(DownloadErrors::DownloadCorrapt("ff".to_string()))
                // let tmp_dir = Builder::new().tempdir().map_err(|er| ConfigErrors::LoadCorrapt(er.to_string()));
                let _ = match Vanila::find(&*ver).await {
                    Ok(_) => {},
                    Err(e) => {error!("{:#?}", e)},
                };
                
                todo!()
            },
        }
    }
}