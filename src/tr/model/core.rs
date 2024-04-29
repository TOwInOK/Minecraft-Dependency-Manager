use crate::errors::error::Result;
use crate::settings::core::Core;
use crate::tr::hash::ChooseHash;

pub trait ModelCore {
    type Link;
    type Version;
    fn get_link(
        core: &Core,
    ) -> impl std::future::Future<Output = Result<(Self::Link, ChooseHash, Self::Version)>> + Send;
}
