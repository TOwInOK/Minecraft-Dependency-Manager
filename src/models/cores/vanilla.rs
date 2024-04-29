use crate::{
    errors::error::{Error, Result},
    not_found_version_error,
    settings::core::Core,
    tr::{hash::ChooseHash, model::core::ModelCore},
};
use serde::Deserialize;
use serde::Serialize;

type OuterLink = String;
type VersionID = String;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vanilla {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        // warn!("Use default fn of TypeOfVersion");
        TypeOfVersion::Release
    }
}

//Area of download from list of details about version

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadSection {
    pub downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Downloads {
    pub server: Server,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    pub sha1: String,
    pub url: String,
}

impl ModelCore for Vanilla {
    type Link = OuterLink;
    type Version = VersionID;
    /// Making request to mojang api and find the link to download minecraft.jar
    async fn get_link(core: &Core) -> Result<(OuterLink, ChooseHash, VersionID)> {
        let version = core.version();
        // debug!("Start find fn with version: {:#?}", version);
        let link = find_version(version).await?;
        // trace!("get link: {}", &link.0);
        let response = reqwest::get(link.0).await?;
        // trace!("get response, status of request: {}", &response.status());
        let download_section: DownloadSection = response.json().await?;
        // debug!("Find jar to download!");
        // debug!("Check body: {:#?}", &download_section.downloads.server);
        Ok((
            download_section.downloads.server.url,
            ChooseHash::SHA1(download_section.downloads.server.sha1),
            link.1,
        ))
    }
}

///Return `url` which get a json that contain links of all versions
async fn find_version(version: &str) -> Result<(String, String)> {
    const LINK: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    // trace!("Start find version of core!");
    let response = reqwest::get(LINK).await?;
    let vanilla: Vanilla = response.json().await?;
    let local_version = {
        if version == "Latest" {
            vanilla.latest.release
        } else {
            version.to_string()
        }
    };
    // info!("Need to find: {}", &local_version);

    // Use a temporary variable to hold the found version and URL
    let found_version_and_url = vanilla
        .versions
        .iter()
        .find(|x| x.version.contains(&local_version))
        .map(|x| {
            // info!("find version: {}", &x.version);
            let c = x.version.clone();
            (c, x.url.clone())
        });

    // Update `version` outside of the closure
    if let Some((version_str, url)) = found_version_and_url {
        Ok((url, version_str))
    } else {
        not_found_version_error!(local_version)
    }
}
