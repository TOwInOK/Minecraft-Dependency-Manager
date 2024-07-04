pub mod core;
pub mod ext;
pub mod ext_metalist;

use indicatif::ProgressBar;
use log::debug;
use serde::{Deserialize, Serialize};

use self::core::CoreMeta;
use self::ext_metalist::ExtensionMetaList;
use crate::{
    settings::core::Core,
    tr::{load::Load, save::Save},
};
use crate::{DICTIONARY, SETTINGS};
use std::sync::Arc;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Lock {
    core: CoreMeta,
    plugins: ExtensionMetaList,
    mods: ExtensionMetaList,
}
impl Lock {
    pub fn core(&self) -> &CoreMeta {
        &self.core
    }
    pub fn plugins(&self) -> &ExtensionMetaList {
        &self.plugins
    }
    pub fn core_mut(&mut self) -> &mut CoreMeta {
        &mut self.core
    }
    pub fn plugins_mut(&mut self) -> &mut ExtensionMetaList {
        &mut self.plugins
    }
    pub fn set_core(&mut self, value: Core, build: String) {
        self.core = value.to_meta(build);
    }
    pub async fn remove_plugin(&mut self, key: &str) {
        self.plugins.remove(key).await
    }
    pub async fn remove_mod(&mut self, key: &str) {
        self.mods.remove(key).await
    }
    //delete and make the current core the default
    pub async fn remove_core(&mut self) {
        self.core.remove().await;
    }
    pub async fn remove_defunct(&mut self, pb: Arc<ProgressBar>) {
        debug!(
            "fn() remove_nonexistent => keys list: {:#?}",
            &self.plugins().0
        );
        pb.set_message(DICTIONARY.defunct().start_remove_defunct());

        // delete for core you can found in Core::download()
        if let Some(settings_plugins) = SETTINGS.read().await.plugins() {
            let lock_list = self.plugins().get_list().clone();
            for (key, _) in lock_list {
                if !settings_plugins.items().contains_key(&key) {
                    pb.set_message(DICTIONARY.defunct().remove_if_defunct(&key));
                    debug!("{}", DICTIONARY.defunct().remove_if_defunct(&key));
                    self.remove_plugin(&key).await
                }
            }
        }

        // TODO: add remover for mods
    }
}

impl Save for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Load for Lock {
    const PATH: &'static str = "./lock.toml";
}
