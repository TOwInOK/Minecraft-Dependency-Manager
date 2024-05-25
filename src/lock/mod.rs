pub mod core;
pub mod ext;

use indicatif::ProgressBar;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use self::core::CoreMeta;
use self::ext::ExtensionMeta;
use crate::settings::Settings;
use crate::tr::delete::Delete;
use crate::{
    settings::core::Core,
    tr::{load::Load, save::Save},
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::dictionary::pb_messages::PbMessages;
use lazy_static::lazy_static;

lazy_static! {
    static ref DICT: PbMessages = PbMessages::load_sync().unwrap();
}

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
        //remove
        self.core = CoreMeta::default();
    }
    pub async fn remove_nonexistent(
        &mut self,
        settings: Arc<RwLock<Settings>>,
        pb: Arc<ProgressBar>,
    ) {
        debug!(
            "fn() remove_nonexistent => keys list: {:#?}",
            &self.plugins().0
        );
        pb.set_message(&DICT.start_remove_nonexist);
        if let Some(settings_plugins) = settings.read().await.plugins() {
            let lock_list = self.plugins().get_list().clone();
            for (key, _) in lock_list {
                if !settings_plugins.items().contains_key(&key) {
                    pb.set_message(DICT.remove_if_nonexist(&key));
                    debug!("{}", DICT.remove_if_nonexist(&key));
                    self.remove_plugin(&key).await
                }
            }
        }
        // TODO: add remover for mods
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ExtensionMetaList(HashMap<String, ExtensionMeta>);

impl ExtensionMetaList {
    pub fn get(&self, key: &str) -> Option<&ExtensionMeta> {
        self.0.get(key)
    }
    pub fn insert(&mut self, key: String, value: ExtensionMeta) {
        self.0.insert(key, value);
    }
    // make it traited
    pub async fn remove(&mut self, key: &str) {
        if let Some(e) = self.0.remove(key) {
            self.delete(e.path()).await
        }
    }
    pub async fn update(&mut self, key: String, value: ExtensionMeta) {
        self.remove(&key).await;
        self.insert(key, value);
    }
    pub fn get_list(&self) -> &HashMap<String, ExtensionMeta> {
        &self.0
    }
}

impl Save for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Load for Lock {
    const PATH: &'static str = "./lock.toml";
}
impl Delete for ExtensionMetaList {}
