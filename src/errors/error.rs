use thiserror::Error;

use crate::mananger::messages::Messages;

#[derive(Error, Debug)]
pub enum CompareHashError {
    #[error("Хэш не совпадает: expect => {0}, value => {1}")]
    HashNotCompare(String, String),
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
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Hash corrupted: {0}")]
    CompareHash(CompareHashError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error{0}")]
    TomlParse(String),
    #[error("Serialization of TOML error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    // #[error("Any error: {0}")]
    // Any(#[from] Box<dyn std::error::Error + Send>),
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

        let message = if parts.len() >= 4 {
            let third_part: String = parts[3]
                .trim()
                .chars()
                .filter(|x| *x != '^' || *x != '\\')
                .collect();
            format!(
                "   Where => {} ||| What => {} ||| why => {}   ",
                parts[0].trim(),
                parts[2].trim(),
                third_part
            )
        } else {
            value.to_string()
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
#[macro_export]
macro_rules! not_found_plugin_link_error {
    ($arg:tt) => {
        Err(Error::NotFound(format!(
            "Not found link in for plugin: ->{}<-",
            $arg
        )))
    };
}
