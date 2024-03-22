use crate::config::lock::ExistState;
use crate::config::lock::Meta;
use crate::config::lock::MetaData;
use crate::config::versions::Versions;
use crate::downloader::hash::ChooseHash;
use crate::downloader::Downloader;
use crate::errors::errors::ConfigErrors;
use crate::errors::errors::DownloadErrors;
use log::debug;
use log::info;
use log::trace;
use log::warn;
use serde::Deserialize;
use serde::Serialize;

use super::model::ModelCore;

type OuterLink = String;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vanilla {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "id")]
    pub version: String,

    #[serde(rename = "type")]
    pub type_field: TypeOfVersion,

    pub url: String,
}

///Minecraft types of version
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeOfVersion {
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}

impl Default for TypeOfVersion {
    fn default() -> Self {
        warn!("Use default fn of TypeOfVersion");
        TypeOfVersion::Release
    }
}

//Area of download from list of details about version

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSection {
    pub downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub server: Server,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub sha1: String,
    pub url: String,
}

impl ModelCore for Vanilla {
    /// Making request to mojang api and find the link to download minecraft.jar
    async fn find(version: &Versions, build: &str) -> Result<(OuterLink, ChooseHash), ConfigErrors> {
        info!("Start find fn with version: {:#?}", &version);
        let link = Self::find_version(version).await?;
        trace!("get link: {}", &link);
        let response = reqwest::get(link).await?;
        trace!("get response, status of request: {}", &response.status());
        let download_section: DownloadSection = response.json().await?;
        info!("Find jar to download!");
        debug!("Check body: {:#?}", &download_section.downloads.server);

        Ok((
            download_section.downloads.server.url,
            ChooseHash::SHA1(download_section.downloads.server.sha1),
        ))
    }

    ///Return `url` for get a json which contain links of all versions
    async fn find_version(version: &Versions) -> Result<String, ConfigErrors> {
        const LINK: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
        trace!("Start find version of core!");
        let response = reqwest::get(LINK).await?;
        let vanilla: Vanilla = response.json().await?;
        let local_version: &String = match version {
            Versions::Version(e) => e,
            Versions::Latest => &vanilla.latest.release,
        };
        info!("Need to find: {}", &local_version);
        vanilla
            .versions
            .iter()
            .find(|x| x.version.contains(local_version))
            .map(|x| {
                info!("find version: {}", &x.version);
                x.url.clone()
            })
            .ok_or_else(|| {
                ConfigErrors::LoadCorrupt(format!(
                    "No one version like: {}, not found",
                    local_version
                ))
            })
    }

    async fn download<'config, 'lock>(downloader: &mut Downloader<'config, 'lock>) -> Result<(), DownloadErrors> {
        let core_name = "Vanilla";
        info!("Find {}!", core_name);
        //Find version to download
        let (link, hash) = Vanilla::find(&downloader.config.core.version, &downloader.config.core.build).await?;

        debug!("Find {} link: {}, hash: {}",core_name, &link, &hash);
        info!("Start to download core!");
        //Need to update or download?
        match downloader.lock.exist(&Meta::Core(MetaData { name: core_name.to_string(), version: downloader.config.core.version.clone() })).await {
            ExistState::Exist => {
                info!("Check freeze and force_update");
                if downloader.config.core.freeze && !downloader.config.core.force_update{
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


