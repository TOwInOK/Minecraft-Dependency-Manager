use crate::errors::error::Result;
use crate::tr::hash::ChooseHash;

pub trait ModelExtensions {
    type Ext;
    type Link;
    type Version;
    fn get_link(
        ext: &Self::Ext,
        name: &str,
        game_version: Option<&String>,
        loader: &str,
    ) -> impl std::future::Future<Output = Result<(Self::Link, ChooseHash, Self::Version)>> + Send;
}
