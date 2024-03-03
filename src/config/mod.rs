mod datapack;
mod errors;
mod models;
mod plugin;
mod version;

use datapack::*;
use errors::*;
use log::{error, info};
use models::vanilla::Vanilla;
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

    pub fn default() -> Self {
        Config::new(Versions::default(), None, None)
    }

    pub async fn load_config(path: String) -> Result<Config, ConfigErrors> {
        info!("Загрузка конфигурационного файла...");
        let toml = fs::read_to_string(&path).await?;
        info!("Файл успешно загружен.");

        info!("Инициализация конфигурационного файла...");
        let config: Config = toml::from_str(&toml)?;
        info!("Конфигурация успешно инициализированна.");

        Ok(config)
    }

    pub async fn download(self) -> Result<(), DownloadErrors> {
        //download core
        let file = self.download_core().await;
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
    async fn download_core(self) -> Result<Option<String>, DownloadErrors> {
        match self.version {
            //Download purpur

            //Download Vanilla
            Versions::Vanilla(ver, freeze) => {
                if freeze {
                    //We don't need to download
                    return Ok(None);
                }
                //use if error
                // Err(DownloadErrors::DownloadCorrupt("ff".to_string()))
                // let tmp_dir = Builder::new().temp().map_err(|er| ConfigErrors::LoadCorrupt(er.to_string()));
                match Vanilla::find(&*ver).await {
                    Ok(_) => {
                        todo!()
                    }
                    Err(e) => {
                        error!("{:#?}", e);
                        Err(e.into())
                    }
                }
            }
            Versions::Purpur(_, _) => todo!(),
            Versions::Paper(_, _) => todo!(),
            Versions::Spigot(_, _) => todo!(),
            Versions::Bucket(_, _) => todo!(),
        }
    }
}
