mod hash;
mod downloader;
mod models;

use std::collections::HashMap;
use log::{debug, info};

use crate::{config::{additions::Additions, core::{Core, Provider}, plugins::{Plugin, Sources}, Config}, errors::errors::DownloadErrors};
use self::{hash::ChooseHash, models::vanilla::Vanilla};

type Name = String;
type Link = String;

//Query for download

#[derive(Debug)]
pub struct Downloader();

impl Downloader {

    pub async fn new () -> Self {
        Self {}
    }
    
    pub async fn check(self, config: &mut Config) -> Result<(), DownloadErrors> {
        info!("Start check fn");
       self.check_core(&config.core, &config.additions.path_to_core).await?;
    //    self.check_plugins(&config.plugins).await?;
        todo!()
    }

    ///Check core and add it into list for download.
    async fn check_core(self, core: &Core, path: &String) -> Result<(), DownloadErrors> {
        info!("Check freeze and force_update");
        if core.freeze && !core.force_update {return Ok(());};
        info!("Start to match provider of core");
        match &core.provider {
            Provider::Vanilla => {
                info!("Find vanilla!");
                let (link,hash) = Vanilla::find(&core.version).await?;
                debug!("Find vanilla link: {}, hash: {}", &link, &hash);
                info!("Start to download core!");
                self.download_core("Vanilla", link, hash, path).await
            },
            Provider::Bukkit => todo!(),
            Provider::Spigot => todo!(),
            Provider::Paper => todo!(),
            Provider::Purpur => todo!(),
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
        }
    }
    ///Check plugins and add it into list for download.
    async fn check_plugins(&mut self, plugins: &HashMap<String, Plugin>) -> Result<(), DownloadErrors> {
        if plugins.is_empty() {return Ok(());};

        for (name,plugin) in plugins.iter() {
            if plugin.freeze && !plugin.force_update {return Ok(());};
            match plugin.source {
                Sources::Bukkit => todo!(),
                Sources::Spigot => todo!(),
                Sources::Hangar => todo!(),
                Sources::Modrinth => todo!(),
                Sources::CurseForge => todo!(),
            }
        } 
        todo!()
    }
    async fn download_core(self, name: &str, link: String, hash: ChooseHash, download_dir: &String) -> Result<(), DownloadErrors> {
        get_file(link, hash, download_dir).await?;
        todo!()
    }
    async fn download_plugin(self) -> Result<(), DownloadErrors> {
        todo!()
    }
}

use std::path::Path;
use std::fs::File;
use std::io::Write;

async fn get_file(link: String, hash: ChooseHash, download_dir: &str) -> Result<(), DownloadErrors> {
    let response = reqwest::get(&link).await?;
    let content = response.bytes().await?;

    // Проверка хеша
    if hash.calculate_hash(&*content).await {
        let file_name = Path::new(&link).file_name().unwrap_or_default(); // Name of file
        debug!("File name: {:#?}", &file_name);
        let file_path = Path::new(download_dir).join(file_name); // Where to download with file name
        debug!("File path: {:#?}", &file_path);
        let mut file = File::create(&file_path)?; // Create the file
        file.write_all(&content)?; //write
        Ok(())
    } else {
        Err(DownloadErrors::DownloadCorrupt("Hash doesn't match".to_string()))
    }
}