use crate::tr::load::Load;
use std::collections::HashMap;
use std::sync::Arc;

use futures_util::future::join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use super::plugin::Plugin;
use crate::dictionary::pb_messages::PbMessages;
use crate::errors::error::Result;
use crate::lock::ext::ExtensionMeta;
use crate::lock::Lock;
use crate::pb;
use crate::tr::hash::ChooseHash;
use crate::tr::{download::Download, save::Save};

lazy_static! {
    static ref DICT: PbMessages = PbMessages::load_sync().unwrap();
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub struct Plugins(HashMap<String, Plugin>);

impl Plugins {
    pub fn new(items: HashMap<String, Plugin>) -> Self {
        Self(items)
    }

    pub fn items(&self) -> &HashMap<String, Plugin> {
        &self.0
    }

    pub async fn download_all(
        &self,
        loader: &str,
        game_version: &str,
        lock: Arc<Mutex<Lock>>,
        mpb: Arc<MultiProgress>,
    ) -> Result<()> {
        let link_list = self.check_plugins(game_version, loader, mpb, &lock).await?;
        let handler_list = make_handle_list(link_list, lock)?;
        join_all(handler_list).await;
        Ok(())
    }

    /// Check lock extensions with config extesions
    async fn check_plugins(
        &self,
        game_version: &str,
        loader: &str,
        mpb: Arc<MultiProgress>,
        lock: &Arc<Mutex<Lock>>,
    ) -> Result<Vec<(String, ChooseHash, String, String, ProgressBar)>> {
        let mut link_list = Vec::new();
        for (name, plugin) in self.0.iter() {
            debug!("check extension: {}", &name);
            // Get link
            let (link, hash, build) = plugin.get_link(name, game_version, loader).await?;
            debug!("got a link to the extesion: {}", &name);
            let pb = pb!(mpb, name);
            debug!("check meta: {}", &name);
            // Check meta
            if let Some(plugin_meta) = lock.lock().await.plugins().get(name) {
                let local_build = plugin_meta.build();
                // Need to download?
                if *local_build == build && !plugin.force_update() || plugin.freeze() {
                    debug!("Does't need to update: {}", &name);
                    pb.set_message(&DICT.doest_need_to_update);
                    pb.finish_and_clear();
                    continue;
                }
            }
            debug!("add link to list: {}", &name);
            link_list.push((link, hash, build, name.to_owned(), pb))
        }
        Ok(link_list)
    }
}

/// Create list with futures to download
fn make_handle_list(
    link_list: Vec<(String, ChooseHash, String, String, ProgressBar)>,
    lock: Arc<Mutex<Lock>>,
) -> Result<Vec<JoinHandle<Result<()>>>> {
    let mut handler_list: Vec<JoinHandle<Result<()>>> = Vec::new();
    for (link, hash, build, name, pb) in link_list {
        let lock = Arc::clone(&lock);
        handler_list.push(tokio::spawn(async move {
            // get file
            let file = Plugin::get_file(link, hash, &pb).await?;

            debug!("Remove exist version of {}", &name);
            {
                pb.set_message(&DICT.delete_exist_version);
                lock.lock().await.remove_plugin(&name).await;
            }
            debug!("Saving {}", &name);

            pb.set_message(&DICT.saving_file);
            Plugin::save_bytes(file, &name).await?;

            debug!("Write data to lock file {}", &name);

            pb.set_message(&DICT.write_to_lock);
            {
                lock.lock()
                    .await
                    .plugins_mut()
                    .update(name.to_string(), {
                        ExtensionMeta::new(build, Plugin::PATH, &name)
                    })
                    .await;
            }
            debug!("Save meta data to lock of {}", &name);

            lock.lock().await.save().await?;
            pb.set_message(&DICT.done);

            pb.finish_and_clear();
            Ok(())
        }));
    }
    Ok(handler_list)
}
