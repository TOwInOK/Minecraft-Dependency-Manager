use crate::{
    config::versions::Versions, downloader::hash::ChooseHash, errors::errors::ConfigErrors,
};

pub trait ModelCore {
    async fn find(version: &Versions) -> Result<(String, ChooseHash), ConfigErrors>;
    async fn find_version(version: &Versions) -> Result<String, ConfigErrors>;
}
