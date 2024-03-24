use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub block_time_start: u32, // Time in minutes
    pub block_time_end: u32,
}

pub fn get_configs() -> Result<Config, ConfigError> {
    web_sys::window()
        .ok_or(ConfigError::StorageNotFound)?
        .local_storage()
        .map_err(|_| ConfigError::WontAllowStorage)?
        .expect("Calling .local_storage() should never return null/None, according to MDN")
        .get_item("config")
        .expect("Calling .get_item() should never throw, only return None if item doesn't exist or the item, according to MDN")
        .ok_or(ConfigError::EmptyStorage)
        .and_then(|item| {
            serde_json::from_str(&item).map_err(|_| ConfigError::CorruptedConfig)
        })
}

pub fn set_configs(config: &Config) -> Result<(), ConfigError> {
    web_sys::window()
        .ok_or(ConfigError::StorageNotFound)?
        .local_storage()
        .map_err(|_| ConfigError::WontAllowStorage)?
        .expect("Calling .local_storage() should never return null/None, according to MDN")
        .set_item(
            "config",
            &serde_json::to_string(config).map_err(|_| ConfigError::CorruptedConfig)?,
        )
        .map_err(|_| ConfigError::WontAllowStorage) // Either this or QuotaExceeded. Both cases are effectively
                                                    // that user's actions have denied me this operation, so it's all the same to me.
}

pub fn remove_configs() -> Result<(), ConfigError> {
    web_sys::window()
        .ok_or(ConfigError::StorageNotFound)?
        .local_storage()
        .map_err(|_| ConfigError::WontAllowStorage)?
        .expect("Calling .local_storage() should never return null/None, according to MDN")
        .remove_item("config") // NOTE: This method wouldn't throw if key isn't present. It just wouldn't do anything.
        .map_err(|_| ConfigError::WontAllowStorage)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfigError {
    WontAllowStorage,
    EmptyStorage,
    StorageNotFound,
    CorruptedConfig,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StorageError: ")?;
        let msg = match self {
            ConfigError::WontAllowStorage => "The user has not allowed storage",
            ConfigError::EmptyStorage => "The storage is empty",
            ConfigError::StorageNotFound => "The window context/storage context was not found",
            ConfigError::CorruptedConfig => "The config is corrupted",
        };
        writeln!(f, "{msg}")
    }
}
impl Error for ConfigError {}
