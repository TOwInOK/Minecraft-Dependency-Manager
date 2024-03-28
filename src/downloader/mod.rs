pub mod hash;
mod models;

use crate::config::core::Provider;
use crate::config::plugins::{Plugin, Sources};
use crate::config::Config;
use crate::downloader::models::cores::folia::Folia;
use crate::downloader::models::cores::paper::Paper;
use crate::downloader::models::cores::purpur::Purpur;
use crate::downloader::models::cores::vanilla::Vanilla;
use crate::downloader::models::cores::velocity::Velocity;
use crate::downloader::models::cores::waterfall::Waterfall;
use crate::downloader::models::model::ModelCore;
use crate::errors::error::DownloadErrors;
use crate::lock::lock::{ExistState, Lock, Meta, MetaData};

use log::{debug, info};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use self::hash::ChooseHash;

#[derive(Debug)]
pub struct Downloader<'config, 'lock> {
    config: &'config mut Config,
    lock: &'lock mut Lock,
}

impl<'config, 'lock> Downloader<'config, 'lock> {
    pub fn new(config: &'config mut Config, lock: &'lock mut Lock) -> Self {
        Self { config, lock }
    }

    ///Check and download plugins, mods, core
    pub async fn check(&mut self) -> Result<(), DownloadErrors> {
        info!("Start check fn");
        self.core_reqwest().await
        // self.plugin_reqwest().await
    }

    ///Check core and add it into list for download.
    async fn get_core_link(&mut self) -> Result<(String, ChooseHash), DownloadErrors> {
        info!("Start to match provider of core");
        match &self.config.core.provider {
            Provider::Vanilla => Vanilla::find(&mut self.config.core).await,
            Provider::Paper => Paper::find(&mut self.config.core).await,
            Provider::Folia => Folia::find(&mut self.config.core).await,
            Provider::Purpur => Purpur::find(&mut self.config.core).await,
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
            Provider::Waterfall => Waterfall::find(&mut self.config.core).await,
            Provider::Velocity => Velocity::find(&mut self.config.core).await,
        }
    }

    /// Make reqwest to check version and download core.
    async fn core_reqwest(&mut self) -> Result<(), DownloadErrors> {
        //Find version to download
        let (link, hash) = self.get_core_link().await?;
        let core_name = &*self.config.core.provider.get_name().await;
        debug!("Find {} link: {}, hash: {}", core_name, &link, &hash);
        info!("Start to download {}!", core_name);
        //Need to update or download?
        match self
            .lock
            .exist(&Meta::Core(MetaData {
                name: core_name.to_string(),
                version: self.config.core.version.clone(),
                build: self.config.core.build.clone().into(),
                dependencies: None,
            }))
            .await
        {
            ExistState::Exist => {
                if self.config.core.freeze().await {
                    return Ok(());
                }
                Self::force_update(self, core_name, link, hash).await
            }
            ExistState::DifferentVersion | ExistState::DifferentBuild => {
                if self.config.core.freeze().await {
                    return Ok(());
                }
                info!("Core have different or build version, Download!");
                self.download_core(core_name, link, hash).await
            }
            ExistState::None => {
                info!("No one core find, Download!");
                self.download_core(core_name, link, hash).await
            }
        }
    }

    async fn force_update(
        &mut self,
        core_name: &str,
        link: String,
        hash: ChooseHash,
    ) -> Result<(), DownloadErrors> {
        if self.config.core.force_update {
            self.config.core.force_update = false;
            info!("Force update core!");
            return self.download_core(core_name, link, hash).await;
        }
        info!("Core doesn't need to download");
        Ok(())
    }
    /// download core
    async fn download_core(
        &mut self,
        name: &str,
        link: String,
        hash: ChooseHash,
    ) -> Result<(), DownloadErrors> {
        //delete all cores from meta and dir
        self.lock
            .delete_core(&self.config.additions.path_to_core)
            .await?;
        //download
        get_file(link, hash, &self.config.additions.path_to_core, name).await?;

        //get meta data
        let meta = Meta::Core(MetaData::new(
            name.to_string(),
            self.config.core.version.clone(),
            self.config.core.build.clone().into(),
            None,
        ));
        //push to lock
        self.lock.add(meta).await;
        //save lock
        self.lock.save(&self.config.additions.path_to_lock).await?;
        Ok(())
    }

    /// Make reqwest to check version and download [`Plugin`].
    async fn plugin_reqwest(&mut self) -> Result<(), DownloadErrors> {
        if self.config.plugins.is_empty() {
            return Ok(());
        };
        for (name, plugin) in self.config.plugins.iter_mut() {
            if plugin.freeze().await {
                continue;
            }
            let (link, hash) = Self::get_plugin_link(name, plugin).await?;

            // Check exist plugin.
            // match self
            // .lock
            // .exist(&Meta::Plugin(MetaData {
            //     name: core_name.to_string(),
            //     version: self.config.core.version.clone(),
            //     build: self.config.core.build.clone().into(),
            //     dependencies: None,
            // }))
            // .await? {}
            //
            Self::download_plugin(self.lock, name, plugin).await? // use it inside
        }
        Ok(())
    }

    /// download plugin
    async fn download_plugin(
        lock: &mut Lock,
        name: &str,
        plugin: &Plugin,
    ) -> Result<(), DownloadErrors> {
        todo!()
    }

    ///Check plugins and add it into list for download.
    async fn get_plugin_link(
        name: &str,
        plugin: &Plugin,
    ) -> Result<(String, ChooseHash), DownloadErrors> {
        match plugin.source {
            Sources::Spigot => todo!(),
            Sources::Hangar => todo!(),
            Sources::Modrinth => todo!(),
            Sources::CurseForge => todo!(),
        }
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
        info!("Hash same");
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
