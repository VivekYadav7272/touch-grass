use std::error::Error;

use crate::console_log;
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

    let page = match get_configs() {
        Ok(config) => show_settings(cx, config),
        Err(ConfigError::EmptyStorage) => show_welcome_screen(cx),
        Err(ConfigError::StorageNotFound) => render!(
            h3 {
                "Storage bucket not found! :( It either seems like you are either using a very old browser
                or you're not running it in one"
            }
        ),
        Err(ConfigError::WontAllowStorage) => render!(
            h3 {
                "You need to allow storage for this extension to work! It might be that you've disabled use of cookies"
            }
        ),
        Err(ConfigError::CorruptedConfig) => {
            // We need to remove the config and then show the welcome screen
            remove_configs().expect("Couldn't set the default config");

            render!(
                h3 {
                    "The config is corrupted! We're gonna have to start from scratch! Try re-opening the extension popup"
                }
            )
        }
    };

    render!(
        link { rel: "stylesheet", href: "./output.css" }
        page
    )
}

fn show_welcome_screen(cx: Scope) -> Element {
    let start_hour: &UseState<Option<u32>> = use_state(cx, || None);
    let start_minute: &UseState<Option<u32>> = use_state(cx, || None);
    let end_hour: &UseState<Option<u32>> = use_state(cx, || None);
    let end_minute: &UseState<Option<u32>> = use_state(cx, || None);

    render!(
        div { class: "rounded-lg border bg-card text-card-foreground shadow-sm w-full max-w-sm mx-auto",
            div { class: "flex flex-col space-y-1.5 p-6",
                h3 { class: "font-semibold whitespace-nowrap tracking-tight text-lg", "TouchGrass" }
                p { class: "text-sm text-muted-foreground",
                    "Control your YouTube usage with this extension."
                }
            }
            div { class: "p-6 grid gap-4",
                div { class: "flex items-center",
                    label {
                        class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                        r#for: "start-time",
                        "Start Time"
                    }
                }
                div { class: "flex items-center",
                    input {
                        class: "flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-16",
                        id: "start-hour",
                        placeholder: "08",
                        r#type: "number",

                        oninput: move |evt| {
                            start_hour.set(evt.value.parse::<u32>().ok().filter(|hr| *hr < 24));
                        }
                    }
                    span { class: "mx-1", ":" }
                    input {
                        class: "flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-16",
                        id: "start-minute",
                        placeholder: "00",
                        r#type: "number",
                        oninput: move |evt| {
                            start_minute.set(evt.value.parse::<u32>().ok().filter(|mnt| *mnt < 60));
                        }
                    }
                }
                div { class: "flex items-center",
                    label {
                        class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                        r#for: "end-time",
                        "End Time"
                    }
                }
                div { class: "flex items-center",
                    input {
                        class: "flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-16",
                        id: "end-hour",
                        placeholder: "17",
                        r#type: "number",
                        oninput: move |evt| {
                            end_hour.set(evt.value.parse::<u32>().ok().filter(|hr| *hr < 24));
                        }
                    }
                    span { class: "mx-1", ":" }
                    input {
                        class: "flex h-10 rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-16",
                        id: "end-minute",
                        placeholder: "00",
                        r#type: "number",
                        oninput: move |evt| {
                            end_minute.set(evt.value.parse::<u32>().ok().filter(|mnt| *mnt < 60));
                        }
                    }
                }
                button { class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-white hover:bg-primary/90 h-10 px-4 py-2 w-full",
                    "Save"
                }
            }
            div { class: "flex items-center p-6",
                p { class: "text-xs text-gray-500 dark:text-gray-400",
                    "YouTube will be disabled between the specified times."
                }
            }
        }
    )
}

fn show_settings(cx: Scope, config: Config) -> Element {
    render!( h1 { "Speaking from show_settings!" } )
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
