use serde::{Deserialize, Serialize};

use crate::{
    errors::error::{Error, Result},
    not_found_build_error, not_found_version_error,
    settings::core::Core,
    tr::{hash::ChooseHash, model::core::ModelCore},
};

pub struct Purpur {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VersionList {
    versions: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildList {
    builds: Builds,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Builds {
    latest: String,
    all: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FileHash {
    md5: String,
}

// Download
// https://api.purpurmc.org/v2/purpur/{Version}/{Build}/download
//
const MAIN_LINK: &str = "https://api.purpurmc.org/v2/purpur";

impl ModelCore for Purpur {
    type Link = String;

    type Version = String;

    //find build and push link
    async fn get_link(core: &Core) -> Result<(String, ChooseHash, String)> {
        let build = core.build();
        let version = core.version();
        let version = find_version(version).await?;
        //Version string
        let verlink = format!("{}/{}", MAIN_LINK, version);
        let build_list: BuildList = reqwest::get(verlink).await?.json().await?;
        let build_list_latest: &str = build_list.builds.latest.as_ref();
        let build_list = build_list.builds.all;

        match build {
            Some(build) => {
                if build_list.contains(&build.to_owned()) {
                    let build_link = format!("{}/{}/{}", MAIN_LINK, version, build);
                    let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
                    Ok((
                        format!("{}/download", build_link),
                        ChooseHash::MD5(file_hash.md5),
                        build.to_owned(),
                    ))
                } else {
                    not_found_build_error!(build)
                }
            }
            None => {
                let build_link = format!("{}/{}/{}", MAIN_LINK, version, build_list_latest);
                let file_hash: FileHash = reqwest::get(&build_link).await?.json().await?;
                Ok((
                    format!("{}/download", build_link),
                    ChooseHash::MD5(file_hash.md5),
                    build_list_latest.to_owned(),
                ))
            }
        }
    }
}

//Find version in version list, if exist give out version or give error
async fn find_version(version: &str) -> Result<String> {
    let version_list = {
        let version_list: VersionList = reqwest::get(MAIN_LINK).await?.json().await?;
        version_list.versions
    };
    match version {
        "Latest" => match version_list.last() {
            Some(e) => Ok(e.to_owned()),
            None => not_found_version_error!("Latest"),
        },
        _ => {
            if version_list.contains(&version.to_string()) {
                Ok(version.to_owned())
            } else {
                not_found_version_error!(version)
            }
        }
    }
}
