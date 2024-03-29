use crate::{
    config::{core::Core, versions::Versions},
    downloader::{hash::ChooseHash, models::model::ModelCore, Downloader},
    errors::error::{ConfigErrors, DownloadErrors},
    lock::lock::{ExistState, Meta, MetaData},
};
use log::debug;
use log::info;
use log::trace;
use log::warn;
use serde::Deserialize;
use serde::Serialize;
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
    async fn get_link(core: &mut Core) -> Result<(OuterLink, ChooseHash), DownloadErrors> {
        let version = &core.version;
        info!("Start find fn with version: {:#?}", version);
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
    async fn find_version(version: &Versions) -> Result<String, DownloadErrors> {
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
                DownloadErrors::DownloadCorrupt(format!(
                    "No one version like: {}, not found",
                    local_version
                ))
            })
    }
}
