use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    config::{core::Core, models::model::ModelCore},
    downloader::hash::ChooseHash,
    errors::error::{Error, Result},
    not_found_build_error, not_found_version_error,
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
    async fn get_link(core: &Core) -> Result<(String, ChooseHash, String)> {
        let build = core.build.as_deref();
        let version = core.version.as_deref();
        let version = Self::find_version(version).await?;
        //Version string
        let verlink = format!("https://api.purpurmc.org/v2/purpur/{}", version);
        info!("Get BuildList");
        let build_list: BuildList = reqwest::get(verlink).await?.json().await?;
        let build_list_latest: &str = build_list.builds.latest.as_ref();
        let build_list = build_list.builds.all;

        match build {
            Some(e) => {
                if build_list.contains(&e.to_owned()) {
                    info!("Find build, download");
                    let build_link =
                        format!("https://api.purpurmc.org/v2/purpur/{}/{}", version, e);
                    info!("Get Url");
                    let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
                    Ok((
                        format!("{}/download", build_link),
                        ChooseHash::MD5(file_hash.md5),
                        e.to_owned(),
                    ))
                } else {
                    not_found_build_error!(e)
                }
            }
            None => {
                info!("Download latest build");
                info!("Get Url");
                let build_link = format!(
                    "https://api.purpurmc.org/v2/purpur/{}/{}",
                    version, build_list_latest
                );
                let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
                Ok((
                    format!("{}/download", build_link),
                    ChooseHash::MD5(file_hash.md5),
                    build_list_latest.to_owned(),
                ))
            }
        }
    }

    //Find version in version list, if exist give out version or give error
    async fn find_version(version: Option<&str>) -> Result<String> {
        let link = "https://api.purpurmc.org/v2/purpur";
        let verlist: VersionList = reqwest::get(link).await?.json().await?;
        let verlist: &[String] = verlist.versions.as_ref();
        match version {
            Some(ver) => {
                if verlist.contains(&ver.to_owned()) {
                    Ok(ver.to_owned())
                } else {
                    not_found_version_error!(ver)
                }
            }
            None => match verlist.last() {
                Some(e) => Ok(e.to_string()),
                None => not_found_version_error!("Latest"),
            },
        }
    }
}
