use async_trait::async_trait;
use log::warn;
use tokio::fs;

#[async_trait]
pub trait Delete {
    /// Delete file from the file system
    async fn delete(&self, path: &str) {
        if !path.is_empty() {
            match fs::remove_file(path).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("IO error: {}, with path: {}", e, path)
                }
            };
        }
    }
}
