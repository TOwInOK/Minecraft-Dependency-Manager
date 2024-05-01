use crate::errors::error::{Error, Result};
use crate::not_found_path;
use bytes::Bytes;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Сохраняем структуру на диск
pub trait Save {
    const PATH: &'static str;
    fn save(&self) -> impl std::future::Future<Output = Result<()>> + Send
    where
        Self: serde::ser::Serialize + Sync,
    {
        async move {
            {
                // Сериализуем в понятный человеку томл
                let toml_content = toml::to_string_pretty(&self)?;

                // Откроем для записи
                let mut file = File::create(Self::PATH).await?;

                // Запишем файл
                file.write_all(toml_content.as_bytes()).await?;

                Ok(())
            }
        }
    }
    fn save_bytes(
        bytes: Bytes,
        name: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            {
                // Конвертируем &str в Path
                let path_to_dir = Path::new(Self::PATH);
                // Проверить путь на исправность
                if !path_to_dir.exists() {
                    return not_found_path!(Self::PATH);
                }
                // Добавляем имя к пути
                let path_to_file = path_to_dir.join(format!("{}.jar", name));

                // Откроем для записи
                let mut file = File::create(path_to_file).await?;

                // Запишем файл
                file.write_all(&bytes).await?;

                Ok(())
            }
        }
    }
}
