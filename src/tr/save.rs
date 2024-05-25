use crate::errors::error::{Error, Result};
use crate::not_found_path;
use async_trait::async_trait;
use bytes::Bytes;
use log::debug;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Сохраняем структуру на диск
#[async_trait]
pub trait Save {
    const PATH: &'static str;
    // Save data into file.
    async fn save(&self) -> Result<()>
    where
        Self: serde::ser::Serialize + Sync,
    {
        // Сериализуем в понятный человеку томл
        let toml_content = toml::to_string_pretty(&self)?;
        debug!("Save content: {}", Self::PATH);
        // Откроем для записи
        let mut file = File::create(Self::PATH).await?;

        // Запишем файл
        file.write_all(toml_content.as_bytes()).await?;

        Ok(())
    }
    // Create file by data
    async fn save_bytes(bytes: Bytes, name: &str) -> Result<()> {
        // Конвертируем &str в Path
        let path_to_dir = Path::new(Self::PATH);
        // Проверить путь на исправность
        if !path_to_dir.exists() {
            return not_found_path!(Self::PATH);
        }
        debug!("path to dir: {:#?}", path_to_dir.to_str());
        // Добавляем имя к пути
        let path_to_file = path_to_dir.join(format!("{}.jar", name));

        debug!("path to file: {:#?}", path_to_file.to_str());
        // Откроем для записи
        let mut file = File::create(path_to_file).await?;

        // Запишем файл
        file.write_all(&bytes).await?;

        Ok(())
    }
}
