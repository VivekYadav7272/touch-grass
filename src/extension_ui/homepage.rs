use crate::{config, console_log};
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    dioxus::launch(app)
}

fn app() -> Element {
    // First we need to check if the user has even setup the extension or not.
    // Depending on that, we either render the welcome screen or the normal setting screen.
    use config::ConfigError;
    let page = use_resource(move || async move {
        match config::get_config().await {
            Ok(config) => show_settings(Some(config)),
            Err(ConfigError::EmptyStorage) => show_welcome_screen(),
            Err(ConfigError::StorageNotFound) => rsx!(
                h3 {
                    "Storage bucket not found! :( It either seems like you are either using a very old browser
                    or you're not running it in one"
                }
            ),
            Err(ConfigError::WontAllowStorage) => rsx!(
                h3 {
                    "You need to allow storage for this extension to work!"
                }
            ),
            Err(ConfigError::CorruptedConfig) => {
                // We need to remove the config and then show the welcome screen
                config::remove_config()
                    .await
                    .expect("Couldn't set the default config");

                rsx!(
                    h3 {
                        "The config is corrupted! We're gonna have to start from scratch! Try re-opening the extension popup"
                    }
                )
            }
        }
    });

    if let Some(page) = page().flatten() {
        rsx! {
            link { rel: "stylesheet", href: "./output.css" }
            {page}
        }
    } else {
        rsx! { "Loading..." }
    }
}

fn parse_time(time: &str) -> Option<u32> {
    time.split_once(':').and_then(|(hour, minute)| {
        let hour = hour.parse::<u32>().ok()?;
        let minute = minute.parse::<u32>().ok()?;
        Some(hour * 60 + minute)
    })
}

fn show_welcome_screen() -> Element {
    show_settings(None)
}

fn show_settings(config: Option<config::Config>) -> Element {
    // Idea for this page:
    // 1. Show the current settings (i.e start and end time)
    // 2. Allow the user to change the settings by reverting to the previous page.
    // 3. Show some statistics (hours of YouTube accessed today, etc.)

    let block_time_start = config.as_ref().map(|config| config.block_time_start);
    let block_time_end = config.as_ref().map(|config| config.block_time_end);

    let mut start_time: Signal<Option<u32>> = use_signal(|| block_time_start);
    let mut end_time: Signal<Option<u32>> = use_signal(|| block_time_end);

    rsx!(
        div { class: "rounded-lg border bg-card text-card-foreground shadow-sm w-full max-w-sm mx-auto",
            div { class: "flex flex-col space-y-1.5 p-6",
                h3 { class: "font-semibold whitespace-nowrap tracking-tight text-lg", "TouchGrass" }
                p { class: "text-sm text-muted-foreground",
                    "Control your YouTube usage with this extension."
                }
            }
            div { class: "p-6 grid gap-4",
                div { class: "flex flex-row justify-between",
                    div { class: "flex flex-col",
                        div { class: "flex items-center",
                            label {
                                class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                                r#for: "start-time",
                                "Start Time"
                            }
                        }
                        div { class: "flex items-center",
                            input {
                                class: "flex h-10 rounded-md border border-input bg-background mt-2 px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-23",
                                class: if start_time.read().is_none() { "red-border" },
                                id: "start-time",
                                placeholder: "08",
                                value: if let Some(start_time) = block_time_start {
                                    format!("{:02}:{:02}", start_time / 60, start_time % 60)
                                },
                                r#type: "time",

                                oninput: move |evt| start_time.set(parse_time(&evt.value()))
                            }
                        }
                    }
                    div { class: "flex flex-col",
                        div { class: "flex items-center",
                            label {
                                class: "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
                                r#for: "end-time",
                                "End Time"
                            }
                        }
                        div { class: "flex items-center",
                            input {
                                class: "flex h-10 rounded-md border border-input bg-background mt-2 px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-23",
                                class: if end_time.read().is_none() { "red-border" },
                                id: "end-time",
                                placeholder: "17",
                                value: if let Some(end_time) = block_time_end {
                                    format!("{:02}:{:02}", end_time / 60, end_time % 60)
                                },
                                r#type: "time",
                                oninput: move |evt| {
                                    end_time.set(parse_time(&evt.value()));
                                    console_log!("End time is now: {:?}", evt.value());
                                }
                            }
                        }
                    }
                }
                button {
                    class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-white hover:bg-primary/90 h-10 px-4 py-2 w-full",
                    onclick: move |_| {

                        let (Some(start_time), Some(end_time)) = (*start_time.read(), *end_time.read()) else {
                            return;
                        };

                        spawn(async move { config::update_config(|config| {
                            config.block_time_start = start_time;
                            config.block_time_end = end_time;
                        }).await.expect("Couldn't set the config"); });
                    },
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
