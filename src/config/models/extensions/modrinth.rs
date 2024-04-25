use log::info;
use serde::Deserialize;
use serde::Serialize;

use crate::config::models::model::ModelExtensions;
use crate::downloader::hash::ChooseHash;
use crate::errors::error::{Error, Result};
use crate::not_found_plugin_error;

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
pub struct File {
    pub hashes: Hashes,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub project_id: String,
    pub dependency_type: String,
}

impl ModelExtensions for ModrinthData {
    async fn get_link(
        name: &str,
        plugin: &crate::config::plugins::Plugin,
        game_version: Option<&str>,
    ) -> Result<(String, crate::downloader::hash::ChooseHash, String)> {
        let loader = "fabric";
        let link: String = {
            // TODO: Make normal params!
            match game_version {
                Some(game_version) => {
                    let channel = plugin.channel.get_str().await;
                    let link = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]&featured=true&version_type={}", name, game_version, loader, channel);
                    info!("Modrinth link: {}", &link);
                    link
                }
                None => {
                    let channel = plugin.channel.get_str().await;
                    let link = format!("https://api.modrinth.com/v2/project/{}/version?&loaders=[\"{}\"]&featured=true&version_type={}", name, loader, channel);
                    info!("Modrinth link: {}", &link);
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
