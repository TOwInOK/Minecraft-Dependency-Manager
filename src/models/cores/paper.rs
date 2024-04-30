use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};

use crate::{
    errors::error::{Error, Result},
    not_found_build_error, not_found_version_error,
    settings::core::Core,
    tr::{hash::ChooseHash, model::core::ModelCore},
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
struct VersionList {
    versions: Vec<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BuildList {
    builds: Vec<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Url {
    downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Downloads {
    application: Application,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Application {
    name: String,
    sha256: String,
}

impl<T: ModelCorePaperFamily> ModelCore for T {
    type Link = String;
    type Version = String;
    //find build and push link
    async fn get_link(core: &Core) -> Result<(String, ChooseHash, String)> {
        let core_name = Self::CORE_NAME;
        // make pb
        let pb = ProgressBar::new_spinner();
        let name_closure = move |_: &ProgressState, f: &mut dyn std::fmt::Write| {
            f.write_str(core_name).unwrap();
        };
        pb.set_style(
            ProgressStyle::with_template(
                "Package:: {name:.blue} >>> {spinner:.green} {msg:.blue} ",
            )
            .unwrap()
            .with_key("name", name_closure),
        );
        // Start work
        pb.set_message("Init work");
        //get data from core
        let build = core.build();
        let version = core.version();
        //find link and version

        pb.set_message("Finding version");

        let version = find_version(version, core_name).await?;
        let verlink = format!(
            "https://api.papermc.io/v2/projects/{}/versions/{}",
            core_name, version
        );

        pb.set_message("Make link");

        let build_list: BuildList = reqwest::get(verlink).await?.json().await?;
        let build_list = build_list.builds.as_slice();
        match build {
            Some(e) => {
                let build: u16 = e.parse().unwrap();
                if build_list.contains(&build) {
                    let buildlink = format!(
                        "https://api.papermc.io/v2/projects/{}/versions/{}/builds/{}",
                        core_name, version, build
                    );
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
                let lastbuild = build_list.last().unwrap();
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
}

//Find version in version list, if exist give out version or give error
async fn find_version(version: &str, core_name: &str) -> Result<String> {
    let link = format!("https://api.papermc.io/v2/projects/{}", core_name);
    let version_list = {
        let version_list: VersionList = reqwest::get(link).await?.json().await?;
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
