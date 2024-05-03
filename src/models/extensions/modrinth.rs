use log::debug;
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
    id: String,
    //Stable token.
    project_id: String,
    files: Vec<File>,
    dependencies: Vec<Dependency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct File {
    hashes: Hashes,
    url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Hashes {
    sha1: String,
    sha512: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Dependency {
    project_id: String,
    dependency_type: String,
}

impl ModelExtensions for ModrinthData {
    type Ext = Plugin;
    type Link = String;
    type Version = String;
    async fn get_link(
        ext: &Self::Ext,
        name: &str,
        game_version: &str,
        loader: &str,
    ) -> Result<(Self::Link, ChooseHash, Self::Version)> {
        let link: String = {
            // TODO: Make normal params!
            match game_version {
                "Latest" => {
                    let channel = ext.channel().get_str().await;
                    let link = format!("https://api.modrinth.com/v2/project/{}/version?&loaders=[\"{}\"]&featured=true&version_type={}", name, loader, channel);
                    debug!("Plugin: {} -> Link: {}", name, link);
                    link
                }
                _ => {
                    let channel = ext.channel().get_str().await;
                    let link = format!("https://api.modrinth.com/v2/project/{}/version?game_versions=[\"{}\"]&loaders=[\"{}\"]&featured=true&version_type={}", name, game_version, loader, channel);
                    debug!("Plugin: {} -> Link: {}", name, link);
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
