use super::config::Config;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
