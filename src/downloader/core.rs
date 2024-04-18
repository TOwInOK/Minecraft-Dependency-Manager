use log::{debug, info};

use crate::config::core::{Core, Provider};
use crate::downloader::models::cores::folia::Folia;
use crate::downloader::models::cores::paper::Paper;
use crate::downloader::models::cores::purpur::Purpur;
use crate::downloader::models::cores::vanilla::Vanilla;
use crate::downloader::models::cores::velocity::Velocity;
use crate::downloader::models::cores::waterfall::Waterfall;
use crate::downloader::models::model::ModelCore;
use crate::errors::error::DownloadErrors;
use crate::lock::lock::{ExistState, Lock, Meta, MetaData};

use super::hash::ChooseHash;


pub trait CoreDownloader {
    ///Check core and add it into list for download.
async fn get_core_link(core: &mut Core) -> Result<(String, ChooseHash), DownloadErrors> {
    info!("Start to match provider of core");
    match core.provider {
        Provider::Vanilla => Vanilla::get_link(core).await,
        Provider::Paper => Paper::get_link(core).await,
        Provider::Folia => Folia::get_link(core).await,
        Provider::Purpur => Purpur::get_link(core).await,
        // Provider::Fabric => todo!(),
        // Provider::Forge => todo!(),
        // Provider::NeoForge => todo!(),
        Provider::Waterfall => Waterfall::get_link(core).await,
        Provider::Velocity => Velocity::get_link(core).await,
    }
}

/// Make reqwest to check version and download core.
async fn core_reqwest(core: &mut Core, lock: &mut Lock) -> Result<(), DownloadErrors> {
    //Find version to download
    let (link, hash) = Self::get_core_link(core).await?;
    let core_name = core.provider.get_name().await.as_str();
    debug!("Find {} link: {}, hash: {}", core_name, &link, &hash);
    info!("Start to download {}!", core_name);
    //Need to update or download?
    match lock
        .exist(&Meta::Core(MetaData {
            name: core_name.to_string(),
            version: core.version,
            build: core.build.into(),
            dependencies: None,
        }))
        .await
    {
        ExistState::Exist => {
            if core.freeze().await {
                return Ok(());
            }
            Self::force_update(self, core_name, link, hash).await
        }
        ExistState::DifferentVersion | ExistState::DifferentBuild => {
            if core.freeze().await {
                return Ok(());
            }
            info!("Core have different or build version, Download!");
            Self::download_core(core_name, link, hash).await
        }
        ExistState::None => {
            info!("No one core find, Download!");
            Self::download_core(core_name, link, hash).await
        }
    }
}

async fn force_update(
    lock: &mut Lock,
    core: &Core,
    link: String,
    hash: ChooseHash,
) -> Result<(), DownloadErrors> {
    if core.force_update {
        core.force_update = false;
        info!("Force update core!");
        return Self::download_core(core.provider.get_name(), link, hash).await;
    }
    info!("Core doesn't need to download");
    Ok(())
}
/// download core
async fn download_core(
    lock: &mut Lock,
    name: &str,
    link: String,
    hash: ChooseHash,
) -> Result<(), DownloadErrors> {
    //delete all cores from meta and dir
    lock
        .delete_core(&self.config.additions.path_to_core).await?;
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

}
