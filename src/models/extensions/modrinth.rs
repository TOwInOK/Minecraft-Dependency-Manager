use log::trace;
use serde::Deserialize;
use serde::Serialize;

use crate::errors::error::{Error, Result};
use crate::not_found_plugin_error;
use crate::not_found_plugin_link_error;
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
    // project_id: String,
    files: Vec<File>,
    // dependencies: Vec<Dependency>,
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
    async fn get_link(
        ext: &Self::Ext,
        name: &str,
        game_version: Option<&String>,
        loader: &str,
    ) -> Result<(String, ChooseHash, String)> {
        let channel = ext.channel().get_str().await.to_string();
        let link = format!("https://api.modrinth.com/v2/project/{}/version", name);

        let loader = {
            if loader.to_lowercase() == "purpur" {
                "paper"
            } else {
                loader
            }
        };
        let query = {
            match game_version {
                None => {
                    vec![
                        // ("game_version", format!("[\"{}\"]", game_version)),
                        ("loaders", format!("[\"{}\"]", loader)),
                        ("featured", true.to_string()),
                        ("version_type", channel),
                    ]
                }

                Some(e) => {
                    vec![
                        ("game_version", format!("[\"{}\"]", e)),
                        ("loaders", format!("[\"{}\"]", loader)),
                        ("featured", true.to_string()),
                        ("version_type", channel),
                    ]
                }
            }
        };
        trace!("query: {:#?}", &query);
        let client = reqwest::Client::builder()
            .user_agent("TOwInOK/Minecraft-Dependency-Controller (TOwInOK@nothub.ru) TestPoligon")
            .build()?;

        let modrinth_data: Vec<ModrinthData> =
            client.get(&link).query(&query).send().await?.json().await?;
        let modrinth_data = match modrinth_data.first() {
            Some(e) => Ok(e),
            None => not_found_plugin_error!(name),
        }?;
        match modrinth_data.files.first().map(|x| {
            (
                x.url.to_string(),
                ChooseHash::SHA1(x.hashes.sha1.to_string()),
                modrinth_data.id.to_owned(),
            )
        }) {
            Some(e) => Ok(e),
            None => not_found_plugin_link_error!(name),
        }
    }
}
