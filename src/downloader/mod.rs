mod hash;
mod downloader;

use std::collections::HashMap;
use crate::{config::{core::{Core, Provider}, plugins::{Plugin, Sources}, Config}, errors::errors::DownloadErrors};
use self::hash::ChooseHash;

type Name = String;
type Link = String;

struct Downloader {
    //List for download 
    download_list: Vec<Type>,
}

///name, link, hash
enum Type {
    Plugin(Name, Link, ChooseHash),
    Core(Name, Link, ChooseHash),
}

impl Downloader {

    pub async fn check(&mut self, config: &mut Config) -> Result<(), DownloadErrors> {
       self.check_core(&config.core).await?;
       self.check_plugins(&config.plugins).await?;
        todo!()
    }

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

    ///Check core and add it into list for download.
    async fn check_core(&mut self, core: &Core) -> Result<(), DownloadErrors> {
        if core.freeze && !core.force_update {return Ok(());};

        match &core.provider {
            Provider::Vanilla => todo!(),
            Provider::Bucket => todo!(),
            Provider::Spigot => todo!(),
            Provider::Paper => todo!(),
            Provider::Purpur => todo!(),
            Provider::Fabric => todo!(),
            Provider::Forge => todo!(),
            Provider::NeoForge => todo!(),
        }
    }
    // async fn download_mods(config: &Mods) {
    //     todo!();
    // }
    ///Check plugins and add it into list for download.
    async fn check_plugins(&mut self, plugins: &HashMap<String, Plugin>) -> Result<(), DownloadErrors> {
        if plugins.is_empty() {return Ok(());};

        for (name,plugin) in plugins.iter() {
            if plugin.freeze && !plugin.force_update {return Ok(());};
            match plugin.source {
                Sources::Bucket => todo!(),
                Sources::Spigot => todo!(),
                Sources::Hangar => todo!(),
                Sources::Modrinth => todo!(),
                Sources::CurseForge => todo!(),
            }
        } 
        todo!()
    }
}


impl Sources {

}