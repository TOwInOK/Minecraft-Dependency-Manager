use crate::errors::errors::CompareHashError;
use md5::Md5;
use sha1::Digest as Digest1;
use sha1::Sha1;
use sha2::Digest as Digest256;
use sha2::Sha256;
use tokio::io::AsyncReadExt;



#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum ChooseHash {
    SHA1(String),
    SHA256(String),
    MD5(String),
}

impl ChooseHash {
    pub async fn calculate_hash(
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
