use crate::{
    config::versions::Versions, downloader::{hash::ChooseHash, Downloader}, errors::errors::{ConfigErrors, DownloadErrors},
};

pub trait ModelCore {
    async fn find(version: &Versions, build: &str) -> Result<(String, ChooseHash), ConfigErrors>;
    async fn find_version(version: &Versions) -> Result<String, ConfigErrors>;
    async fn download(downloader: &mut Downloader) -> Result<(), DownloadErrors>;
}
