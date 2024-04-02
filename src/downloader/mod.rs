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
use crate::lock::lock::{ExistState, Lock};

use log::{debug, info};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use self::hash::ChooseHash;
use self::models::extensions::modrinth::ModrinthData;
use self::models::model::ModelExtensions;

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
    pub async fn check_and_download(&mut self) -> Result<(), DownloadErrors> {
        info!("Start check fn");
        self.core_reqwest().await?;
        self.plugin_reqwest().await
    }

    /////Core section

    ///Check core and add it into list for download.
    async fn get_core_link(&self) -> Result<(String, ChooseHash, String), DownloadErrors> {
        info!("Start to match provider of core");
        match self.config.core.provider {
            Provider::Vanilla => Vanilla::get_link(&self.config.core).await,
            Provider::Paper => Paper::get_link(&self.config.core).await,
            Provider::Folia => Folia::get_link(&self.config.core).await,
            Provider::Purpur => Purpur::get_link(&self.config.core).await,
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
            Provider::Waterfall => Waterfall::get_link(&self.config.core).await,
            Provider::Velocity => Velocity::get_link(&self.config.core).await,
        }
    }

    /// Make reqwest to check version and download core.
    async fn core_reqwest(&mut self) -> Result<(), DownloadErrors> {
        //Find version to download
        let (link, hash, version) = self.get_core_link().await?;
        let core_name = self.config.core.provider.get_name().await;
        debug!("Find {} link: {}, hash: {}", core_name, &link, &hash);
        info!("Start to download {}!", core_name);
        //Need to update or download?
        match self.lock.exist_core(&self.config.core, &version).await {
            ExistState::Exist => {
                if self.config.core.freeze().await {
                    return Ok(());
                }
                info!("Core exist");
                self.core_force_update(core_name, link, hash, version).await
            }
            ExistState::DifferentVersion | ExistState::DifferentBuild => {
                if self.config.core.freeze().await {
                    return Ok(());
                }
                info!("Core have different or build version, Download!");
                self.download_core(core_name, link, hash, version).await
            }
            ExistState::None => {
                info!("No one core find, Download!");
                self.download_core(core_name, link, hash, version).await
            }
        }
    }

    async fn core_force_update(
        &mut self,
        core_name: &str,
        link: String,
        hash: ChooseHash,
        version: String,
    ) -> Result<(), DownloadErrors> {
        if self.config.core.force_update {
            self.config.core.force_update = false;
            info!("Force update core!");
            return self.download_core(core_name, link, hash, version).await;
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
        build: String,
    ) -> Result<(), DownloadErrors> {
        // download
        get_file(link, hash, &self.config.additions.path_to_core, name).await?;
        let path = format!("{}/{}.jar", self.config.additions.path_to_core, name);
        self.lock
            .core_edit(self.config.core.clone(), path, build)
            .await;
        //save lock
        self.lock.save(&self.config.additions.path_to_lock).await?;
        Ok(())
    }

    ///////Plugin Section

    /// Make reqwest to check version and download [`Plugin`].
    async fn plugin_reqwest(&mut self) -> Result<(), DownloadErrors> {
        if self.config.plugins.is_empty() {
            return Ok(());
        };
        for (name, plugin) in self
            .config
            .plugins
            .iter()
            .map(|(name, plugin)| (name.to_lowercase().replace('_', "-"), plugin))
        {
            if plugin.freeze().await {
                continue;
            }
            let (link, hash, version) =
                get_plugin_link(&name, plugin, self.config.core.version.as_deref()).await?;

            // Check exist plugin.
            match self.lock.exist_plugin(&name, &version).await {
                ExistState::Exist => {
                    if plugin.freeze().await {
                        return Ok(());
                    }
                    // Download force update
                    if plugin.force_update {
                        info!("Plugin: {}. Force download!", name);
                        download_plugin(
                            self.lock,
                            &name,
                            version,
                            &self.config.additions.path_to_plugins,
                            &self.config.additions.path_to_configs,
                            link,
                            hash,
                        )
                        .await?
                    }
                    info!("Plugin: {}. Does't need to update", name);
                }
                ExistState::DifferentVersion | ExistState::DifferentBuild => {
                    if plugin.freeze().await {
                        return Ok(());
                    }
                    info!("Plugin: {}. Need tp update", name);
                    download_plugin(
                        self.lock,
                        &name,
                        version,
                        &self.config.additions.path_to_plugins,
                        &self.config.additions.path_to_configs,
                        link,
                        hash,
                    )
                    .await?
                }
                ExistState::None => {
                    info!("No one plugin: {} find, Download!", name);
                    download_plugin(
                        self.lock,
                        &name,
                        version,
                        &self.config.additions.path_to_plugins,
                        &self.config.additions.path_to_configs,
                        link,
                        hash,
                    )
                    .await?
                }
            }
        }
        Ok(())
    }
}

/// download plugin
async fn download_plugin(
    lock: &mut Lock,
    name: &str,
    version: String,
    path_plugin: &str,
    path_lock: &str,
    link: String,
    hash: ChooseHash,
) -> Result<(), DownloadErrors> {
    //This downloaded plugin replace current plugin.
    //download plugin
    get_file(link, hash, path_plugin, name).await?;
    //push to lock
    lock.plugin_add(name.to_owned(), version, path_plugin).await;
    //save lock
    lock.save(path_lock).await?;
    Ok(())
}

///Check plugins and add it into list for download.
async fn get_plugin_link(
    name: &str,
    plugin: &Plugin,
    game_version: Option<&str>,
) -> Result<(String, ChooseHash, String), DownloadErrors> {
    match plugin.source {
        Sources::Spigot => todo!(),
        Sources::Hangar => todo!(),
        Sources::Modrinth => ModrinthData::get_link(name, plugin, game_version).await,
        Sources::CurseForge => todo!(),
    }
}
/// Get and write file by path
async fn get_file(
    link: String,
    hash: ChooseHash,
    download_dir: &str,
    name: &str,
) -> Result<(), DownloadErrors> {
    let response = reqwest::get(link).await?;
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
        info!("delete curent core");
        file.write_all(&content)?; //write
        Ok(())
    } else {
        Err(DownloadErrors::DownloadCorrupt(
            "Hash doesn't match".to_string(),
        ))
    }
}
