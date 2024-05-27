use crate::errors::error::CompareHashError;
use crate::errors::error::{Error, Result};
use md5::{Digest as Digest5, Md5};
use serde::{Deserialize, Serialize};
use sha1::{Digest as Digest1, Sha1};
use sha2::{Digest as Digest256, Sha256};
use tokio::io::AsyncReadExt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize, Deserialize)]
pub enum ChooseHash {
    SHA1(String),
    SHA256(String),
    MD5(String),
    None,
}

impl ChooseHash {
    pub async fn calculate_hash(
        &self,
        mut reader: impl tokio::io::AsyncRead + Unpin,
    ) -> Result<()> {
        match self {
            ChooseHash::SHA1(e) => {
                let mut hashed = <Sha1 as Digest1>::new();
                let mut buffer = [0; 32];
                while let Ok(n) = reader.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    hashed.update(&buffer[..n]);
                }
                let result = hashed.finalize();
                if e.eq(&format!("{:x}", result)) {
                    Ok(())
                } else {
                    Err(Error::CompareHash(CompareHashError::HashNotCompare(
                        e.to_owned(),
                        format!("{:x}", result),
                    )))
                }
            }
            ChooseHash::SHA256(e) => {
                let mut hashed = <Sha256 as Digest256>::new();
                let mut buffer = [0; 32];
                while let Ok(n) = reader.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    hashed.update(&buffer[..n]);
                }
                let result = hashed.finalize();
                if e.eq(&format!("{:x}", result)) {
                    Ok(())
                } else {
                    Err(Error::CompareHash(CompareHashError::HashNotCompare(
                        e.to_owned(),
                        format!("{:x}", result),
                    )))
                }
            }
            ChooseHash::MD5(e) => {
                let mut hashed = <Md5 as Digest5>::new();
                let mut buffer = [0; 32];
                while let Ok(n) = reader.read(&mut buffer).await {
                    if n == 0 {
                        break;
                    }
                    hashed.update(&buffer[..n]);
                }
                let result = hashed.finalize();
                if e.eq(&format!("{:x}", result)) {
                    Ok(())
                } else {
                    Err(Error::CompareHash(CompareHashError::HashNotCompare(
                        e.to_owned(),
                        format!("{:x}", result),
                    )))
                }
            }
            ChooseHash::None => Ok(()),
        }
    }
}

impl std::fmt::Display for ChooseHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChooseHash::SHA1(hash) => write!(f, "SHA1: {}", hash),
            ChooseHash::SHA256(hash) => write!(f, "SHA256: {}", hash),
            ChooseHash::MD5(hash) => write!(f, "MD5: {}", hash),
            ChooseHash::None => write!(f, "None hash"),
        }
    }
}
