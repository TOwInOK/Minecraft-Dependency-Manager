use log::info;
use reqwest::Request;
use serde::Deserialize;
use serde::Serialize;

use crate::config::ConfigErrors;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vanila {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    #[serde(skip)]
    pub type_field: String,
    pub url: String,
    #[serde(skip)]
    pub time: String,
    #[serde(skip)]
    pub release_time: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSection {
    pub downloads: Downloads,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    pub server: Server,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub sha1: String,
    pub url: String,
}

impl Vanila {
    ///Making request to mojang api and find the link to download minecraft.jar
    pub async fn find(version: &str) -> Result<(), ConfigErrors> {
        const LINK: &str = "https://piston-meta.mojang.com/v1/packages/8bcd47def18efee744bd0700e86ab44a96ade21f/1.20.4.json";
        let body = match reqwest::get(LINK).await {
            Ok(e) => {match e.json::<DownloadSection>().await {
                Ok(e) => {e.downloads.server},
                Err(e) => {Err(ConfigErrors::LoadCorrapt(e.to_string()))}?,
            }},
            Err(e) => {Err(ConfigErrors::LoadCorrapt(e.to_string()))}?,
        };
        info!("Check body: {:#?}", &body);
        Ok(())
    }

    pub async fn download(link: &str) {
        todo!()
    }
}