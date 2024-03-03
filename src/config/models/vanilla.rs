use log::info;
use log::warn;
use serde::Deserialize;
use serde::Serialize;

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

//

impl Server {
    pub async fn download(self) {
        todo!()
    }
}

impl Vanilla {
    /// Making request to mojang api and find the link to download minecraft.jar
    pub async fn find(version: &str) -> Result<(), ConfigErrors> {
        let link = Vanilla::find_version(version)
            .await
            .map_err(|e| ConfigErrors::LoadCorrupt(e.to_string()))?;
        let response = reqwest::get(link)
            .await
            .map_err(|e| ConfigErrors::LoadCorrupt(e.to_string()))?;
        let download_section: DownloadSection = response
            .json()
            .await
            .map_err(|e| ConfigErrors::LoadCorrupt(e.to_string()))?;

        info!("Check body: {:#?}", &download_section.downloads.server);

        Ok(())
    }

    ///Return `url` for get a json which contain link to donwload
    pub async fn find_version(version: &str) -> Result<String, ConfigErrors> {
        const LINK: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

        let response = reqwest::get(LINK)
            .await
            .map_err(|e| ConfigErrors::LoadCorrupt(e.to_string()))?;
        let vanilla: Vanilla = response
            .json()
            .await
            .map_err(|e| ConfigErrors::LoadCorrupt(e.to_string()))?;

        vanilla
            .versions
            .iter()
            .find(|x| x.version.contains(version))
            .map(|x| x.url.clone())
            .ok_or_else(|| {
                ConfigErrors::LoadCorrupt(format!("No one version like: {}, not found", version))
            })
    }
}
