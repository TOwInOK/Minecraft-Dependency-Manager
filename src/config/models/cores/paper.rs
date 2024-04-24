use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    config::{core::Core, models::model::ModelCore},
    downloader::hash::ChooseHash,
    errors::error::{Error, Result},
    not_found_build_error, not_found_version_error,
};

pub struct Paper();
impl ModelCorePaperFamily for Paper {
    const CORE_NAME: &'static str = "paper";
}

pub trait ModelCorePaperFamily {
    const CORE_NAME: &'static str;
}

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

impl<T: ModelCorePaperFamily> ModelCore for T {
    //find build and push link
    async fn get_link(core: &Core) -> Result<(String, ChooseHash, String)> {
        debug!("Start Get link");
        let core_name = Self::CORE_NAME;
        //get data from core
        let build = core.build.as_deref();
        let version = core.version.as_deref();
        //find link and version
        let version = Self::find_version(version).await?;
        let verlink = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}",
            core_name, version
        );
        info!("Get BuildList");
        let build_list: BuildList = reqwest::get(verlink).await?.json().await?;
        let build_list = build_list.builds.as_slice();
        match build {
            Some(e) => {
                let build: u16 = e.parse().unwrap();
                if build_list.contains(&build) {
                    info!("Find build, download");
                    let buildlink = format!(
                        "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
                        core_name, version, build
                    );
                    info!("Get Url");
                    let url: Url = reqwest::get(&buildlink).await?.json().await?;
                    Ok((
                        format!("{}/downloads/{}", buildlink, url.downloads.application.name),
                        ChooseHash::SHA256(url.downloads.application.sha256),
                        e.to_owned(),
                    ))
                } else {
                    not_found_build_error!(build)
                }
            }
            None => {
                info!("Download latest build");
                let lastbuild = build_list.last().unwrap();
                info!("Get Url");
                let buildlink = format!(
                    "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
                    core_name, version, lastbuild
                );
                let url: Url = reqwest::get(&buildlink).await?.json().await?;
                Ok((
                    format!("{}/downloads/{}", buildlink, url.downloads.application.name),
                    ChooseHash::SHA256(url.downloads.application.sha256),
                    lastbuild.to_string(),
                ))
            }
        }
    }

    //Find version in version list, if exist give out version or give error
    async fn find_version(version: Option<&str>) -> Result<String> {
        debug!("Start find Version");
        let link = format!("https://api.papermc.io/v2/projects/{}", Self::CORE_NAME);
        let verlist: VersionList = reqwest::get(link).await?.json().await?;
        let verlist = verlist.versions;
        match version {
            Some(ver) => {
                if verlist.contains(&ver.to_owned()) {
                    Ok(ver.to_owned())
                } else {
                    not_found_version_error!(ver)
                }
            }
            None => match verlist.last() {
                Some(e) => Ok(e.to_owned()),
                None => not_found_version_error!("Latest"),
            },
        }
    }
}
