use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{downloader::{hash::ChooseHash, models::model::ModelCore, Downloader}, errors::errors::{ConfigErrors, DownloadErrors}, lock::lock::{ExistState, Meta, MetaData}};


pub enum Waterfall{}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionList {
    pub versions: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildList {
    pub builds: Vec<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url {
    pub downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Downloads {
    pub application: Application,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub sha256: String,
}

impl ModelCore for Waterfall {
    //find build and push link
    async fn find(version: &crate::config::versions::Versions, build: &str) -> Result<(String, ChooseHash), ConfigErrors> {
        let version = Self::find_version(version).await?;
        let verlink = format!("https://api.papermc.io/v2/projects/paper/versions/{}", version);
        info!("Get BuildList");
        let buildlist: BuildList  = reqwest::get(verlink).await?.json().await?;
        let buildList = buildlist.builds;
        if !build.is_empty() {
            let build: u16 = build.parse().unwrap();
            if buildList.contains(&build) {
                info!("Find build, download");
                let buildlink = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}",version, build);
                info!("Get Url");
                let url: Url  = reqwest::get(&buildlink).await?.json().await?;
                return Ok((format!("{}/downloads/{}",buildlink, url.downloads.application.name), ChooseHash::SHA256(url.downloads.application.sha256)))
            }
            else {
                Err(ConfigErrors::LoadCorrupt(format!("No one build like: {} find", build)))
            }
        } else {
            info!("Download latest build");
            let lastbuild = buildList.last().unwrap();
            info!("Get Url");
            let buildlink = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}",version, lastbuild);
            let url: Url  = reqwest::get(&buildlink).await?.json().await?;
            return Ok((format!("{}/downloads/{}",buildlink, url.downloads.application.name), ChooseHash::SHA256(url.downloads.application.sha256)))
        }
    }

    //Find version in version list, if exist give out version or give error
    async fn find_version(version: &crate::config::versions::Versions) -> Result<String, ConfigErrors> {
        let link = "https://api.papermc.io/v2/projects/paper";
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
        let core_name = "Waterfall";
        info!("Find {}!", core_name);
        //Find version to download
        let (link, hash) = Waterfall::find(&downloader.config.core.version, &downloader.config.core.build).await?;

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