use serde::Deserialize;
use serde::Serialize;

use crate::errors::error::{Error, Result};
use crate::not_found_plugin_error;
use crate::settings::extensions::plugin::Plugin;
use crate::tr::hash::ChooseHash;
use crate::tr::model::extension::ModelExtensions;

///# Example
///we have cdn like this: `https://cdn.modrinth.com/data/PROJECT_ID/versions/ID/NAME-LOADER-VERSION.jar`
///we can take `[project_id]` -> `AANobbMI`
///we can take `[id]` -> `4GyXKCLd`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModrinthData {
    //Always change ich version
    pub id: String,
    //Stable token.
    pub project_id: String,
    pub files: Vec<File>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct File {
    pub hashes: Hashes,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Dependency {
    pub project_id: String,
    pub dependency_type: String,
}

impl ModelExtensions for ModrinthData {
    type Ext = Plugin;
    type Link = String;
    type Version = String;
    async fn get_link(
        ext: &Self::Ext,
        name: &str,
        game_version: &str,
    ) -> Result<(Self::Link, ChooseHash, Self::Version)> {
        let loader = "fabric";
        let link: String = {
            // TODO: Make normal params!
            match game_version {
                "Latest" => {
                    let channel = ext.channel().get_str().await;
                    let link = format!("https://api.modrinth.com/v2/project/{}/version?&loaders=[\"{}\"]&featured=true&version_type={}", name, loader, channel);
                    link
                }
                _ => {
                    let channel = ext.channel().get_str().await;
                    let link = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]&featured=true&version_type={}", name, game_version, loader, channel);
                    link
                }
            }
        };
        let modrinth_data: Vec<ModrinthData> = reqwest::get(link).await?.json().await?;
        let modrinth_data = match modrinth_data.first() {
            Some(e) => Ok(e),
            None => not_found_plugin_error!(name),
        }?;
        Ok(modrinth_data
            .files
            .first()
            .map(|x| {
                (
                    x.url.to_string(),
                    ChooseHash::SHA1(x.hashes.sha1.to_string()),
                    modrinth_data.id.to_owned(),
                )
            })
            .unwrap())
    }
}
