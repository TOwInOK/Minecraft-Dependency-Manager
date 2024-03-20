use log::debug;
use log::info;
use log::trace;
use log::warn;
use serde::Deserialize;
use serde::Serialize;

use crate::config::downloader::ChooseHash;
use crate::config::ConfigErrors;

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

impl Vanilla {
    /// Making request to mojang api and find the link to download minecraft.jar
    pub async fn find(version: &str) -> Result<(String, ChooseHash), ConfigErrors> {
        let link = Vanilla::find_version(version).await?;
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

    ///Return `url` for get a json which contain link to download
    pub async fn find_version(mut version: &str) -> Result<String, ConfigErrors> {
        const LINK: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

        let response = reqwest::get(LINK).await?;
        let vanilla: Vanilla = response.json().await?;
        if version == "latest" {
            version = &*vanilla.latest.release;
        }
        vanilla
            .versions
            .iter()
            .find(|x| x.version.contains(version))
            .map(|x| {
                info!("find version: {}", &x.version);
                x.url.clone()
            })
            .ok_or_else(|| {
                ConfigErrors::LoadCorrupt(format!("No one version like: {}, not found", version))
            })
    }
}
