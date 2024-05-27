use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::tr::delete::Delete;

use super::ext::ExtensionMeta;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct ExtensionMetaList(pub HashMap<String, ExtensionMeta>);

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
impl Delete for ExtensionMetaList {}
