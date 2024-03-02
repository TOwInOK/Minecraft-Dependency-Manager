use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

pub type ItemRoot = Vec<ItemData>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    #[serde(rename = "game_versions")]
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "author_id")]
    pub author_id: String,
    pub featured: bool,
    pub name: String,
    #[serde(rename = "version_number")]
    pub version_number: String,
    pub changelog: String,
    #[serde(rename = "changelog_url")]
    pub changelog_url: Value,
    #[serde(rename = "date_published")]
    pub date_published: String,
    pub downloads: i64,
    #[serde(rename = "version_type")]
    pub version_type: String,
    pub status: String,
    #[serde(rename = "requested_status")]
    pub requested_status: Value,
    pub files: Vec<File>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    #[serde(rename = "file_type")]
    pub file_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    #[serde(rename = "version_id")]
    pub version_id: Value,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "file_name")]
    pub file_name: Value,
    #[serde(rename = "dependency_type")]
    pub dependency_type: String,
}
