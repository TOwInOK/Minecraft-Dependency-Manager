use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{downloader::{hash::ChooseHash, models::model::ModelCore, Downloader}, errors::errors::{ConfigErrors, DownloadErrors}, lock::lock::{ExistState, Meta, MetaData}};


pub enum Purpur{}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionList {
    pub versions: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildList {
    pub builds: Builds,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Builds {
    pub latest: String,
    pub all: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileHash {
    pub md5: String,
}

// Download
// https://api.purpurmc.org/v2/purpur/{Version}/{Build}/download


impl ModelCore for Purpur {
    //find build and push link
    async fn find(version: &crate::config::versions::Versions, build: &str) -> Result<(String, ChooseHash), ConfigErrors> {
        let version = Self::find_version(version).await?;
        let verlink = format!("https://api.purpurmc.org/v2/purpur/{}", version);
        info!("Get BuildList");
        let build_list: BuildList  = reqwest::get(verlink).await?.json().await?;
        let build_list_latest = build_list.builds.latest;
        let build_list = build_list.builds.all;
        if !build.is_empty() {
            if build_list.contains(&build.to_string()) {
                info!("Find build, download");
                let build_link = format!("https://api.purpurmc.org/v2/purpur/{}/{}",version, build);
                info!("Get Url");
                let file_hash: FileHash  = reqwest::get(&build_link).await?.json().await?;
                return Ok((format!("{}/download", build_link), ChooseHash::MD5(file_hash.md5)))
            }
            else {
                Err(ConfigErrors::LoadCorrupt(format!("No one build like: {} find", build)))
            }
        } else {
            info!("Download latest build");
            info!("Get Url");
            let build_link = format!("https://api.purpurmc.org/v2/purpur/{}/{}",version, build_list_latest);
            let file_hash: FileHash  = reqwest::get(&build_link).await?.json().await?;
            return Ok((format!("{}/download", build_link), ChooseHash::MD5(file_hash.md5)))
        }
    }

    //Find version in version list, if exist give out version or give error
    async fn find_version(version: &crate::config::versions::Versions) -> Result<String, ConfigErrors> {
        let link = "https://api.purpurmc.org/v2/purpur";
        let verlist: VersionList = reqwest::get(link).await?.json().await?;
        let verlist = verlist.versions;
        match version {
            crate::config::versions::Versions::Version(ver) => {
                if verlist.contains(ver) {
                    Ok(ver.to_string())
                } else {
                    Err(ConfigErrors::LoadCorrupt(format!("No one version ->{}<- find", ver)))
                }
            },
            crate::config::versions::Versions::Latest => {
                match verlist.last() {
                    Some(e) => Ok(e.to_string()),
                    None =>  Err(ConfigErrors::LoadCorrupt("No one version find".to_string())),
                }
            },
        }
    }

    async fn download<'config, 'lock>(downloader: &mut Downloader<'config, 'lock>) -> Result<(), DownloadErrors> {
        let core_name = "Purpur";
        info!("Find {}!", core_name);
        //Find version to download
        let (link, hash) = Purpur::find(&downloader.config.core.version, &downloader.config.core.build).await?;

        debug!("Find {} link: {}, hash: {}",core_name, &link, &hash);
        info!("Start to download core!");
        //Need to update or download?
        match downloader.lock.exist(&Meta::Core(MetaData { name: core_name.to_string(), version: downloader.config.core.version.clone() })).await {
            ExistState::Exist => {
                info!("Check freeze and force_update");
                if downloader.config.core.freeze && !downloader.config.core.force_update {
                    info!("Core has iced");
                    return Ok(());
                };
                if downloader.config.core.force_update {
                    downloader.config.core.force_update = false;
                    info!("Force update core!");
                    return downloader.download_core(core_name, link, hash).await
                } 
                info!("Core doesn't need to download");
                return Ok(());
            },
            ExistState::DifferentVersion => {
                info!("Check freeze and force_update");
                if downloader.config.core.freeze && !downloader.config.core.force_update {
                    info!("Core has iced");
                    return Ok(());
                };
                info!("Core have different version, Download!");
                downloader.download_core(core_name, link, hash).await
            },
            ExistState::None => {
                info!("No one core find, Download!");
                downloader.download_core(core_name, link, hash).await
            },
        }
    }
}