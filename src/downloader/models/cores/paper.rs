use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    config::core::Core,
    downloader::{hash::ChooseHash, models::model::ModelCore},
    errors::error::DownloadErrors,
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
    async fn find(core: &mut Core) -> Result<(String, ChooseHash), DownloadErrors> {
        let core_name = Self::CORE_NAME;
        let build = &mut core.build;
        let version = &core.version;
        let version = Self::find_version(version).await?;
        let verlink = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}",
            core_name, version
        );
        info!("Get BuildList");
        let build_list: BuildList = reqwest::get(verlink).await?.json().await?;
        let build_list = build_list.builds;
        if !build.is_empty() {
            let build: u16 = build.parse().unwrap();
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
                ))
            } else {
                Err(DownloadErrors::DownloadCorrupt(format!(
                    "No one build like: {} find",
                    build
                )))
            }
        } else {
            info!("Download latest build");
            let lastbuild = build_list.last().unwrap().to_string();
            *build = lastbuild;
            info!("Get Url");
            let buildlink = format!(
                "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
                core_name, version, build
            );
            let url: Url = reqwest::get(&buildlink).await?.json().await?;
            Ok((
                format!("{}/downloads/{}", buildlink, url.downloads.application.name),
                ChooseHash::SHA256(url.downloads.application.sha256),
            ))
        }
    }

    //Find version in version list, if exist give out version or give error
    async fn find_version(
        version: &crate::config::versions::Versions,
    ) -> Result<String, DownloadErrors> {
        let link = format!("https://api.papermc.io/v2/projects/{}", Self::CORE_NAME);
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
