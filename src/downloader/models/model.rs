use crate::{
    config::{core::Core, plugins::Plugin, versions::Versions},
    downloader::hash::ChooseHash,
    errors::error::{ConfigErrors, DownloadErrors},
};
use std::collections::{HashMap, HashSet};

pub trait ModelCore {
    async fn get_link(core: &mut Core) -> Result<(String, ChooseHash), DownloadErrors>;
    async fn find_version(version: &Versions) -> Result<String, DownloadErrors>;
}

pub trait ModelExtensions {
    async fn get_link(
        name: &str,
        plugin: &Plugin,
        game_version: &Versions,
        loader: &str,
    ) -> Result<(String, ChooseHash), DownloadErrors>;
}
