use crate::{
    config::{core::Core, versions::Versions},
    downloader::hash::ChooseHash,
    errors::error::{ConfigErrors, DownloadErrors},
    lock::lock::Lock,
};
use std::collections::HashSet;

pub trait ModelCore {
    async fn find(core: &mut Core) -> Result<(String, ChooseHash), DownloadErrors>;
    async fn find_version(version: &Versions) -> Result<String, DownloadErrors>;
}

pub trait ModelPlugins {
    async fn find(version: &Versions, build: &str) -> Result<(String, ChooseHash), ConfigErrors>;
    async fn find_version(version: &Versions) -> Result<String, ConfigErrors>;
    async fn make_download_list(
        lock: &Lock,
        deps: Vec<String>,
        version: &Versions,
    ) -> HashSet<(String, ChooseHash)>;
}
