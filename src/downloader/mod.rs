pub mod hash;
mod models;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use log::{debug, info};

use self::{hash::ChooseHash, models::vanilla::Vanilla};
use crate::{
    config::{
        core::Provider, lock::{Lock, Meta, MetaData}, plugins::Sources, Config
    },
    downloader::models::{model::ModelCore, papermc::Paper},
    errors::errors::DownloadErrors,
};


#[derive(Debug)]
pub struct Downloader<'config, 'lock>{
    config: &'config mut Config,
    lock: &'lock mut Lock,
}

impl<'config, 'lock> Downloader<'config, 'lock>{

    pub fn new(config: &'config mut Config, lock: &'lock mut Lock) -> Self {
        Self { config, lock }
    }
    
    ///Check and download plugins, mods, core
    pub async fn check(&mut self) -> Result<(), DownloadErrors> {
        info!("Start check fn");
        self.check_core().await
        // self.check_plugins(&config.plugins).await?;
    }

    ///Check core and add it into list for download.
    async fn check_core(&mut self) -> Result<(), DownloadErrors> {
        info!("Start to match provider of core");
        match &self.config.core.provider {
            Provider::Vanilla => {
                Vanilla::download(self).await
            }
            Provider::Paper => {
                Paper::download(self).await
            },
            Provider::Folia => todo!(),
            Provider::Purpur => todo!(),
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
        }
    }
    ///Check plugins and add it into list for download.
    async fn check_plugins(
        &self
    ) -> Result<(), DownloadErrors> {
        if self.config.plugins.is_empty() {
            return Ok(());
        };

        for (name, plugin) in self.config.plugins.iter() {
            if plugin.freeze && !plugin.force_update {
                return Ok(());
            };
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
    /// download core
    async fn download_core(
        &mut self,
        name: &str,
        link: String,
        hash: ChooseHash,
    ) -> Result<(), DownloadErrors> {
        self.lock.delete_core(&self.config.additions.path_to_core).await?;
        get_file(link, hash, &self.config.additions.path_to_core, name).await?;
        let meta = Meta::Core(MetaData::new(name.to_string(), self.config.core.version.clone()));
        self.lock.add(meta).await;
        self.lock.save().await?;
        Ok(())
    }
    /// download plugin
    async fn download_plugin(self) -> Result<(), DownloadErrors> {
        todo!()
    }
}

/// Get and write file by path
async fn get_file(
    link: String,
    hash: ChooseHash,
    download_dir: &str,
    name: &str,
) -> Result<(), DownloadErrors> {
    let response = reqwest::get(&link).await?;
    let content = response.bytes().await?;

    // Check hash
    if hash.calculate_hash(&*content).await {
        let mut name = name.to_owned();
        name.push_str(".jar");
        let file_name = Path::new(&name); // Name of file
        debug!("File name: {:#?}", &file_name);
        let file_path = Path::new(download_dir).join(file_name); // Where to download with file name
        debug!("File path: {:#?}", &file_path);
        let mut file = File::create(&file_path)?; // Create the file
        file.write_all(&content)?; //write
        Ok(())
    } else {
        Err(DownloadErrors::DownloadCorrupt(
            "Hash doesn't match".to_string(),
        ))
    }
}
