use crate::config::DownloadErrors;
use log::warn;
use log::{info, trace};
use md5::Md5;
use sha1::Digest as Digest1;
use sha1::Sha1;
use sha2::Digest as Digest256;
use sha2::Sha256;
use std::collections::HashMap;
use std::{env, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::{CompareHashError, ConfigErrors};

pub struct Downloader();

impl Downloader {
    ///Get all info about for download core
    //We need to get sha* or md5 for checking it
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
        let (content, result_str) = Self::get_file(&link, hash.clone()).await?;
        if result_str == hash {
            info!("Файл получен без ошибок!");
            trace!("Hash файла: {}\nHash ожидалось: {:?}", result_str, &hash);
            let current_dir = env::current_dir()?.join("core");
            Self::save_file(content, current_dir, &link).await?;
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
    pub async fn download_plugins(lh: HashMap<String, String>, version: String) -> Result<(), DownloadErrors> {
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
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ChooseHash {
    SHA1(String),
    SHA256(String),
    MD5(String),
}

impl ChooseHash {
    async fn calculate_hash(
        self,
        mut reader: impl tokio::io::AsyncRead + Unpin,
    ) -> Result<Self, CompareHashError> {
        match self {
            ChooseHash::SHA1(_) => {
                let mut hashed = <Sha1 as Digest1>::new();
                let mut buffer = [0; 4096];
                while let Ok(n) = reader.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    hashed.update(&buffer[..n]);
                }
                let result = hashed.finalize();
                Ok(ChooseHash::SHA1(format!("{:x}", result)))
            }
            ChooseHash::SHA256(_) => {
                let mut hashed = <Sha256 as Digest256>::new();
                let mut buffer = [0; 4096];
                while let Ok(n) = reader.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    hashed.update(&buffer[..n]);
                }
                let result = hashed.finalize();
                Ok(ChooseHash::SHA256(format!("{:x}", result)))
            }
            ChooseHash::MD5(_) => {
                let mut hashed = <Md5 as md5::Digest>::new();
                let mut buffer = [0; 4096];
                while let Ok(n) = reader.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    hashed.update(&buffer[..n]);
                }
                let result = hashed.finalize();
                Ok(ChooseHash::MD5(format!("{:x}", result)))
            }
        }
    }
}

impl std::fmt::Display for ChooseHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChooseHash::SHA1(hash) => write!(f, "SHA1: {}", hash),
            ChooseHash::SHA256(hash) => write!(f, "SHA256: {}", hash),
            ChooseHash::MD5(hash) => write!(f, "MD5: {}", hash),
        }
    }
}
