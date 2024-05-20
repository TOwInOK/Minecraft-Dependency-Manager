use thiserror::Error;

use crate::mananger::messages::Messages;

#[derive(Error, Debug)]
pub enum CompareHashError {
    #[error("Хэш не совпадает")]
    HashNotCompare(),
    #[error("Конвертация Sha1 проведена не успешно : {0}")]
    SHA1(std::io::Error),
    #[error("Конвертация Sha256 проведена не успешно : {0}")]
    SHA256(std::io::Error),
    #[error("Конвертация Md5 проведена не успешно : {0}")]
    MD5(std::io::Error),
    #[error("Хэш отсутствует")]
    None,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Проблема с обработкой запроса: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Проблема с обработкой хэша: {0}")]
    CompareHash(CompareHashError),
    // #[error("Проблема с скачиванием файла: {0}")]
    // Download(String),
    #[error("Ошибка ввода/вывода: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    TomlParse(String),
    #[error("Ошибка сериализация TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Не удалось найти: {0}")]
    NotFound(String),
    #[error("Ошибка: {0}")]
    Any(#[from] Box<dyn std::error::Error + Send>),
    #[error("Task join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Indicatif template error: {0}")]
    IndicatifTemplate(#[from] indicatif::style::TemplateError),
    #[error("Indicatif template error: {0}")]
    SendMessage(#[from] tokio::sync::mpsc::error::SendError<Messages>),
}
pub type Result<T> = std::result::Result<T, Error>;

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        let value = value.to_string();
        let parts: Vec<&str> = value.split('|').collect();

        // Проверяем, что индексы существуют и выбираем только нужные
        let message = if parts.len() >= 4 {
            let third_part = parts[3].trim();
            let trimmed_third_part = &third_part[2..]; // Удаляем первые три символа
            format!(
                "   Where => {} ||| What => {} ||| why => {}   ",
                parts[0].trim(),
                parts[2].trim(),
                trimmed_third_part
            )
        } else {
            value.to_string() // Если не удалось разделить на нужное количество частей, вернем исходную строку
        };

        Error::TomlParse(message)
    }
}

#[macro_export]
macro_rules! not_found {
    ($msg:expr) => {
        Err(Error::NotFound($msg.to_string()))
    };
}

#[macro_export]
macro_rules! not_found_path {
    ($arg:expr) => {
        Err(Error::NotFound(format!(
            "No path like: ->{}<-, exist",
            $arg
        )))
    };
}

// #[macro_export]
// macro_rules! download_error {
//     ($($arg:tt)*) => {
//         Err(Error::Download(format!($($arg)*)))
//     };
// }

#[macro_export]
macro_rules! not_found_build_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!(
            "No one build like: ->{}<- find",
            $arg
        )))
    };
}

#[macro_export]
macro_rules! not_found_version_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!("No one version ->{}<- find", $arg)))
    };
}

#[macro_export]
macro_rules! not_found_plugin_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!(
            "No one plugin: ->{}<-, has found.",
            $arg
        )))
    };
}
