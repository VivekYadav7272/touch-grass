use super::config::*;
use wasm_bindgen::JsValue;
use web_extensions_sys::browser;

pub async fn get_configs() -> Result<Config, ConfigError> {
    let storage = browser().storage().local();
    let config_json = storage
        .get(&JsValue::from_str("config"))
        .await
        .map_err(|_| ConfigError::EmptyStorage)?;

    config_json
        .as_string()
        .and_then(|config| serde_json::from_str(&config).ok())
        .ok_or(ConfigError::CorruptedConfig)
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
