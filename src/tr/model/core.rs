use indicatif::ProgressBar;

use crate::errors::error::Result;
use crate::settings::core::Core;
use crate::tr::hash::ChooseHash;

pub trait ModelCore {
    fn get_link(
        core: &Core,
        mpb: &ProgressBar,
    ) -> impl std::future::Future<Output = Result<(String, ChooseHash, String)>> + Send; // Link, Hash, Build
}
