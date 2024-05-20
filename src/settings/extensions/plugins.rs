use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use futures_util::future::join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::errors::error::Result;
use crate::lock::ext::ExtensionMeta;
use crate::lock::Lock;
use crate::tr::{download::Download, save::Save};

use super::plugin::Plugin;

const PATH: &str = "./plugins/";

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
        let mut link_list = Vec::new();
        let mut handler_list: Vec<JoinHandle<Result<()>>> = Vec::new();
        // Make link_list
        // Check plugins in List
        for (name, plugin) in self.0.clone() {
            // Get link
            let (link, hash, build) = plugin.get_link(&name, game_version, loader).await?;
            // PB init
            let pb = mpb.add(ProgressBar::new_spinner());
            // PB style
            pb.set_style(
                ProgressStyle::with_template(
                    "Package:: {prefix:.blue} >>>{spinner:.green} {msg:.blue} > eta: {eta:.blue}",
                )
                .unwrap(),
            );
            pb.set_prefix(name.clone());
            // Check meta
            if let Some(plugin_meta) = lock.lock().await.plugins().get(&name) {
                let local_build = plugin_meta.build();
                // Need to download?
                if *local_build == build && !plugin.force_update() || plugin.freeze() {
                    pb.set_message("Does't need to update");
                    sleep(Duration::from_secs(1)).await;
                    pb.finish_and_clear();
                    continue;
                }
            }
            link_list.push((link, hash, build, name.to_owned(), pb))
        }
        // Make handler_list
        for (link, hash, build, name, pb) in link_list {
            let lock = Arc::clone(&lock);

            handler_list.push(tokio::spawn(async move {
                // get file
                let file = Plugin::get_file(link, hash, &pb).await?;
                pb.set_message("Remove exist version");
                // delete prevision item
                // get lock
                lock.lock().await.remove_plugin(&name)?;
                pb.set_message("Saving...");
                // save on disk
                Plugin::save_bytes(file, name.as_str()).await?;
                pb.set_message("Logging...");
                //save in lock
                lock.lock().await.plugins_mut().insert(name.to_string(), {
                    ExtensionMeta::new(build, format!("{}{}.jar", PATH, name))
                });
                lock.lock().await.save().await?;
                pb.set_message("Done");
                sleep(Duration::from_secs(1)).await;
                pb.finish_and_clear();
                Ok(())
            }));
        }
        join_all(handler_list).await;
        Ok(())
    }
}

impl Download for Plugin {}
impl Save for Plugin {
    const PATH: &'static str = PATH;
}
