use std::error::Error;

use dioxus::{html::h3, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    dioxus_web::launch(app);
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Config {
    block_time_start: u32, // Seconds since the beginning of the day
    block_time_end: u32,
}

fn app(cx: Scope) -> Element {
    // First we need to check if the user has even setup the extension or not.
    // Depending on that, we either render the welcome screen or the normal setting screen.

    match get_configs() {
        Ok(config) => show_settings(cx, config),
        Err(ConfigError::EmptyStorage) => show_welcome_screen(cx),
        Err(ConfigError::StorageNotFound) => render!(
            h3 { "Storage bucket not found! :( It either seems like you are either using a very old browser
                or you're not running it in one"}
        ),
        Err(ConfigError::WontAllowStorage) => render!(
            h3 { "You need to allow storage for this extension to work! It might be that you've disabled use of cookies" }
        ),
        Err(ConfigError::CorruptedConfig) => {
            // We need to remove the config and then show the welcome screen
            remove_configs().expect("Couldn't set the default config");

            render!(
                h3 { "The config is corrupted! We're gonna have to start from scratch! Try re-opening the extension popup" }
            )
        }
    }
}

fn show_welcome_screen(cx: Scope) -> Element {
    render!(
        h1 { "Speaking from show_welcome_screen!"}
    )
}

fn show_settings(cx: Scope, config: Config) -> Element {
    render!(
        h1 { "Speaking from show_settings!"}
    )
}

fn get_configs() -> Result<Config, ConfigError> {
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

fn set_configs(config: &Config) -> Result<(), ConfigError> {
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

fn remove_configs() -> Result<(), ConfigError> {
    web_sys::window()
        .ok_or(ConfigError::StorageNotFound)?
        .local_storage()
        .map_err(|_| ConfigError::WontAllowStorage)?
        .expect("Calling .local_storage() should never return null/None, according to MDN")
        .remove_item("config") // NOTE: This method wouldn't throw if key isn't present. It just wouldn't do anything.
        .map_err(|_| ConfigError::WontAllowStorage)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ConfigError {
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
