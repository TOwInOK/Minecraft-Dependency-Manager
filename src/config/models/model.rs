use crate::{
    config::{core::Core, plugins::Plugin},
    downloader::hash::ChooseHash,
    errors::error::Result,
};

pub trait ModelCore {
    async fn get_link(core: &Core) -> Result<(String, ChooseHash, String)>;
    async fn find_version(version: Option<&str>) -> Result<String>;
}

pub trait ModelExtensions {
    async fn get_link(
        name: &str,
        plugin: &Plugin,
        game_version: Option<&str>,
    ) -> Result<(String, ChooseHash, String)>;
}
