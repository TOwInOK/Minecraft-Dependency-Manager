use crate::errors::error::Result;
use serde::Deserialize;
use tokio::fs;
/// Загружаем с диска нужную нам структуру
pub trait Load {
    const PATH: &'static str;
    fn load() -> impl std::future::Future<Output = Result<Self>> + Send
    where
        for<'de> Self: Deserialize<'de>,
    {
        async move {
            // Читаем с диска
            let file = fs::read_to_string(Self::PATH).await?;
            // Преобразуем в структуру
            let item: Self = toml::from_str(&file)?;
            // Выдача результата
            Ok(item)
        }
    }
}
