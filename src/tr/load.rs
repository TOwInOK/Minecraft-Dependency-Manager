use crate::errors::error::Result;
use async_trait::async_trait;
use log::debug;
use serde::Deserialize;
use tokio::fs::{self};
/// Загружаем с диска нужную нам структуру
#[async_trait]
pub trait Load {
    const PATH: &'static str;

    async fn load() -> Result<Self>
    where
        for<'de> Self: Deserialize<'de>,
    {
        // Читаем с диска
        debug!("Load file: {:#?}", Self::PATH);
        let file = fs::read_to_string(Self::PATH).await?;
        // Преобразуем в структуру
        let item: Self = toml::from_str(&file)?;
        // Выдача результата
        Ok(item)
    }
    fn load_sync() -> Result<Self>
    where
        for<'de> Self: Deserialize<'de>,
    {
        // Читаем с диска
        debug!("Load file: {:#?}", Self::PATH);

        let file = std::fs::read_to_string(Self::PATH)?;
        // Преобразуем в структуру
        let item: Self = toml::from_str(&file)?;
        // Выдача результата
        Ok(item)
    }
}
