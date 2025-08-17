use crate::{config::storage_types::StorageSerdeWrapper, console_log};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen as swb;
use std::error::Error;
use wasm_bindgen::JsValue;
use web_extensions_sys::browser;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub block_time_start: u32, // Time in minutes
    pub block_time_end: u32,
    pub active_days: u8, // bitmap of active days
}

impl Config {
    pub async fn get_config() -> Result<Config, StorageError> {
        Ok(get_storage().await?.user_config)
    }

    pub async fn flush_config(&self) -> Result<(), StorageError> {
        update_storage(|storage| {
            storage.user_config = self.clone();
        })
        .await?;
        Ok(())
    }
}

pub use storage_types::Storage;

// ----------------------------------------------------------------------------------
mod storage_types {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_wasm_bindgen as swb;
    use wasm_bindgen::JsValue;

    #[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
    pub struct Storage {
        pub user_config: Config,
        pub total_usage: u32,
    }

    impl TryFrom<StorageSerdeWrapper> for Storage {
        type Error = StorageError;

        fn try_from(value: StorageSerdeWrapper) -> Result<Self, Self::Error> {
            match value {
                StorageSerdeWrapper::Storage(storage) => Ok(storage),
                StorageSerdeWrapper::EmptyStorage(_) => Err(StorageError::EmptyStorage),
            }
        }
    }

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
    // to {"EmptyStorage": {}}. We just want it to be {} (Because otherwise it will look like
    // JsValue(Object{"EmptyStorage": {}}) which is not what we want. We just want JsValue(Object{})).
    #[derive(Serialize, Deserialize)]
    pub enum StorageSerdeWrapper {
        #[serde(rename = "config")]
        Storage(Storage),
        #[serde(untagged)]
        EmptyStorage(EmptyStruct),
    }

    impl TryFrom<JsValue> for StorageSerdeWrapper {
        type Error = StorageError;

        fn try_from(value: JsValue) -> Result<Self, Self::Error> {
            let value = swb::from_value(value).map_err(|_| StorageError::CorruptedConfig)?;
            Ok(value)
        }
    }
}
// ---------------------------------------------------------------------------

pub async fn get_storage() -> Result<Storage, StorageError> {
    let storage = browser().storage().local();

    let config_jsval = storage
        .get(&JsValue::from_str("config"))
        .await
        .map_err(|_| StorageError::WontAllowStorage)?;

    let config_wrapper = StorageSerdeWrapper::try_from(config_jsval)?;

    Ok(Storage::try_from(config_wrapper)?)
}

pub async fn set_storage(storage: Storage) -> Result<(), StorageError> {
    let browser_storage = browser().storage().local();

    console_log!("[DEBUG]: browser_storage retrieved!");
    let config_jsval = swb::to_value(&StorageSerdeWrapper::Storage(storage)).expect(
        "All types should've been correct because Rust (and its cool static type system(TM)) :)",
    );

    let config_obj = config_jsval.into();

    let _ = browser_storage
        .set(&config_obj)
        .await
        .map_err(|_| StorageError::WontAllowStorage)?;

    Ok(())
}

pub async fn remove_storage() -> Result<(), StorageError> {
    let storage = browser().storage().local();
    let _ = storage
        .remove(&JsValue::from_str("config"))
        .await
        .map_err(|_| StorageError::EmptyStorage)?;
    Ok(())
}

pub async fn update_storage(f: impl FnOnce(&mut Storage)) -> Result<Storage, StorageError> {
    let mut config = get_storage().await.or_else(|err| {
        if err == StorageError::EmptyStorage {
            Ok(Default::default())
        } else {
            Err(err)
        }
    })?;

    f(&mut config);
    set_storage(config.clone()).await?;

    Ok(config)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageError {
    WontAllowStorage,
    EmptyStorage,
    StorageNotFound,
    CorruptedConfig,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StorageError: ")?;
        let msg = match self {
            StorageError::WontAllowStorage => "The user has not allowed storage",
            StorageError::EmptyStorage => "The storage is empty",
            StorageError::StorageNotFound => "The window context/storage context was not found",
            StorageError::CorruptedConfig => "The config is corrupted",
        };
        writeln!(f, "{msg}")
    }
}
impl Error for StorageError {}
