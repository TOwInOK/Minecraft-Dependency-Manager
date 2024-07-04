use crate::errors::error::Result;
use crate::tr::hash::ChooseHash;

pub trait ModelExtensions {
    type Ext;
    fn get_link(
        ext: &Self::Ext,
        name: &str,
        game_version: Option<&str>,
        loader: &str,
    ) -> impl std::future::Future<Output = Result<(String, ChooseHash, String)>> + Send; // Link, Hash, Build
}
