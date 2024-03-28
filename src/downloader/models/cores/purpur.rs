use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    config::core::Core,
    downloader::{hash::ChooseHash, models::model::ModelCore, Downloader},
    errors::error::{ConfigErrors, DownloadErrors},
    lock::lock::{ExistState, Meta, MetaData},
};

pub struct Purpur {}

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
    async fn find(core: &mut Core) -> Result<(String, ChooseHash), DownloadErrors> {
        let version = &core.version;
        let version = Self::find_version(version).await?;
        let build = &mut core.build;
        let verlink = format!("https://api.purpurmc.org/v2/purpur/{}", version);
        info!("Get BuildList");
        let build_list: BuildList = reqwest::get(verlink).await?.json().await?;
        let build_list_latest = build_list.builds.latest;
        let build_list = build_list.builds.all;
        if !build.is_empty() {
            if build_list.contains(&build.to_string()) {
                info!("Find build, download");
                let build_link =
                    format!("https://api.purpurmc.org/v2/purpur/{}/{}", version, build);
                info!("Get Url");
                let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
                Ok((
                    format!("{}/download", build_link),
                    ChooseHash::MD5(file_hash.md5),
                ))
            } else {
                Err(DownloadErrors::DownloadCorrupt(format!(
                    "No one build like: {} find",
                    build
                )))
            }
        } else {
            info!("Download latest build");
            info!("Get Url");
            let build_link = format!(
                "https://api.purpurmc.org/v2/purpur/{}/{}",
                version, build_list_latest
            );
            let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
            *build = build_list_latest.to_string();
            Ok((
                format!("{}/download", build_link),
                ChooseHash::MD5(file_hash.md5),
            ))
        }
    }

    //Find version in version list, if exist give out version or give error
    async fn find_version(
        version: &crate::config::versions::Versions,
    ) -> Result<String, DownloadErrors> {
        let link = "https://api.purpurmc.org/v2/purpur";
        let verlist: VersionList = reqwest::get(link).await?.json().await?;
        let verlist = verlist.versions;
        match version {
            crate::config::versions::Versions::Version(ver) => {
                if verlist.contains(ver) {
                    Ok(ver.to_string())
                } else {
                    Err(DownloadErrors::DownloadCorrupt(format!(
                        "No one version ->{}<- find",
                        ver
                    )))
                }
            }
            crate::config::versions::Versions::Latest => match verlist.last() {
                Some(e) => Ok(e.to_string()),
                None => Err(DownloadErrors::DownloadCorrupt(
                    "No one version find".to_string(),
                )),
            },
        }
    }
}
