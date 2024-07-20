use crate::config::storage_types::ConfigSerdeWrapper;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen as swb;
use std::error::Error;
use wasm_bindgen::JsValue;
use web_extensions_sys::browser;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub block_time_start: u32, // Time in minutes
    pub block_time_end: u32,
}

impl TryFrom<ConfigSerdeWrapper> for Config {
    type Error = ConfigError;

    fn try_from(value: ConfigSerdeWrapper) -> Result<Self, Self::Error> {
        match value {
            ConfigSerdeWrapper::Config(config) => Ok(config),
            ConfigSerdeWrapper::EmptyStorage(_) => Err(ConfigError::EmptyStorage),
        }
    }
}

// ----------------------------------------------------------------------------------
mod storage_types {
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen as swb;
    use wasm_bindgen::JsValue;

    use super::ConfigError;

    #[derive(Serialize, Deserialize)]
    pub struct EmptyStruct {}

    // Here's why we need to do this:
    // browser.storage.local.get() returns a JsValue(Object({})), even if the storage is empty.
    // This is different to how window.localStorage.get() works, where it returns an error if key is not found.
    // I had previously designed assumed that the storage would throw an error if key is not found.
    // But actually, we still get a JsValue. Hence, we instead need to match on the JsValue
    // to decide if it's the error case or not.
    // EmptyStruct is a dummy struct that we use to match on the JsValue.
    // It is untagged because we don't want serde to map ConfigSerdeWrapper::EmptyStorage(EmptyStruct)
    // to be {"EmptyStorage": {}}. We just want it to be {} (Because otherwise it will look like
    // JsValue(Object{"EmptyStorage": {}}) which is not what we want. We just want JsValue(Object{})).
    #[derive(Serialize, Deserialize)]
    pub enum ConfigSerdeWrapper {
        #[serde(rename = "config")]
        Config(super::Config),
        #[serde(untagged)]
        EmptyStorage(EmptyStruct),
    }

    impl TryFrom<JsValue> for ConfigSerdeWrapper {
        type Error = ConfigError;

        fn try_from(value: JsValue) -> Result<Self, Self::Error> {
            let value = swb::from_value(value).map_err(|_| ConfigError::CorruptedConfig)?;
            Ok(value)
        }
    }
}
// ---------------------------------------------------------------------------

pub async fn get_configs() -> Result<Config, ConfigError> {
    let storage = browser().storage().local();

    let config_jsval = storage
        .get(&JsValue::from_str("config"))
        .await
        .map_err(|_| ConfigError::WontAllowStorage)?;

    let config_wrapper = ConfigSerdeWrapper::try_from(config_jsval)?;

    Ok(Config::try_from(config_wrapper)?)
}

pub async fn set_configs(config: Config) -> Result<(), ConfigError> {
    let storage = browser().storage().local();

    let config_jsval = swb::to_value(&ConfigSerdeWrapper::Config(config)).expect(
        "All types should've been correct because Rust (and its cool static type system(TM)) :)",
    );

    let config_obj = config_jsval.into();

    let _ = storage
        .set(&config_obj)
        .await
        .map_err(|_| ConfigError::WontAllowStorage)?;

    Ok(())
}

pub async fn remove_configs() -> Result<(), ConfigError> {
    let storage = browser().storage().local();
    let _ = storage
        .remove(&JsValue::from_str("config"))
        .await
        .map_err(|_| ConfigError::EmptyStorage)?;
    Ok(())
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
