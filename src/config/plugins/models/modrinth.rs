use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::config::downloader::ChooseHash;

pub type Name = String;
pub type Hash = ChooseHash;
pub type ProjectID = String;
pub type PluginID = String;
pub type About = (Hash, ProjectID, PluginID);

pub struct Modrinth {
    plugin: HashMap<Name, About>
}
///# Example
///we have cdn like this: `https://cdn.modrinth.com/data/PROJECT_ID/versions/ID/NAME-LOADER-VERSION.jar`
///we can take `[project_id]` -> `AANobbMI`
///we can take `[id]` -> `4GyXKCLd`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthData {
    //Always change ich version
    pub id: PluginID,
    //Stable token.
    pub project_id: ProjectID,
    pub files: Vec<File>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha1: String,
    // pub sha512: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "version_id")]
    pub version_id: Value,
    #[serde(rename = "file_name")]
    pub file_name: Value,
    #[serde(rename = "dependency_type")]
    pub dependency_type: String,
}


impl Modrinth {
    ///Convert Vector of data to hashmap
    ///for download files
    pub async fn convert(data: Vec<ModrinthData>) -> Self {
        let hashmap: HashMap<Name, About> = HashMap::new();
        data.iter().map(|x| 
            {
             
            });
        todo!()
    }
}