mod datapack;
mod errors;
mod models;
mod plugin;
mod version;
mod downloader;


use downloader::Downloader;
use datapack::*;
use errors::*;
use log::info;
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
        info!("Конфигурация успешно инициализирована.");

        Ok(config)
    }

    pub async fn download_all(self) -> Result<(), DownloadErrors> {
        //download core
        self.choose_core().await
    }

    ///Function download core by info in [`Config`]
    async fn choose_core(self) -> Result<(), DownloadErrors> {
        match self.version {
            //Download vanilla
            Versions::Vanilla(ver, freeze) => {
                let (link, hash) = Vanilla::find(&*ver).await?;
                Downloader::download_core(freeze, link, hash).await
            }
        
            Versions::Purpur(_, _) => todo!(),
            Versions::Paper(_, _) => todo!(),
            Versions::Spigot(_, _) => todo!(),
            Versions::Bucket(_, _) => todo!(),
        }
    }
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

