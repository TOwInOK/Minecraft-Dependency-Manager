use crate::errors::errors::{ConfigErrors, DownloadErrors};
use log::{info, trace, warn};
use std::{collections::HashMap, env, path::PathBuf};
use tokio::io::AsyncWriteExt;

use super::hash::ChooseHash;


pub async fn download_core(
    freeze: bool,
    link: String,
    hash: ChooseHash,
) -> Result<(), DownloadErrors> {
    if freeze {
        warn!("Загрузка ядра была отключена!\n
        Если это ваша первая загрузка, то отключите параметр freeze");
        return Ok(());
    }
    let (content, result_str) = get_file(&link, hash.clone()).await?;
    if result_str == hash {
        info!("Файл получен без ошибок!");
        trace!("Hash файла: {}\nHash ожидалось: {:?}", result_str, &hash);
        let current_dir = env::current_dir()?.join("core");
        save_file(content, current_dir, &link).await?;
        Ok(())
    } else {
        trace!("Hash файла: {}\nHash ожидалось: {:#?}", result_str, &hash);
        Err(DownloadErrors::DownloadCorrupt(
            "Файл получен с ошибками!".to_string(),
        ))
    }
}
///### This func needs to download plugins, not to choose list of plugin to download.
///
///## `[lh]` is map contains values: `[Mode to download, Hash of mod]`
pub fn download_plugins(lh: HashMap<String, String>, version: String) -> Result<(), DownloadErrors> {
    //use version to download file only for this version
    //safe file like `[name-version.jar]`
    todo!()
}

async fn get_file(link: &str, hash: ChooseHash) -> Result<(Vec<u8>, ChooseHash), ConfigErrors> {
    let response = reqwest::get(link).await?;
    let content = response.bytes().await?;
    let hash = hash.calculate_hash(&*content).await?;
    Ok((content.to_vec(), hash))
}

async fn save_file(
    content: Vec<u8>,
    current_dir: PathBuf,
    link: &str,
) -> tokio::io::Result<()> {
    let path_buf = PathBuf::from(link);
    let fname = current_dir.join(
        path_buf
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("tmp.bin")),
    );
    let mut file = tokio::fs::File::create(fname).await?;
    file.write_all(&content).await?;
    Ok(())
}


