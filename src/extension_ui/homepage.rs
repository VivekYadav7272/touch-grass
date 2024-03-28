use crate::{console_log, storage::config};
use dioxus::{html::h3, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    // First we need to check if the user has even setup the extension or not.
    // Depending on that, we either render the welcome screen or the normal setting screen.
    use config::ConfigError;
    let page = match config::get_configs() {
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
            config::remove_configs().expect("Couldn't set the default config");

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

fn parse_time(time: &str) -> Option<u32> {
    time.split_once(':').and_then(|(hour, minute)| {
        let hour = hour.parse::<u32>().ok()?;
        let minute = minute.parse::<u32>().ok()?;
        Some(hour * 60 + minute)
    })
}

fn show_welcome_screen(cx: Scope) -> Element {
    let start_time: &UseState<Option<u32>> = use_state(cx, || None);
    let end_time: &UseState<Option<u32>> = use_state(cx, || None);

    let inp_style = |state: &UseState<Option<u32>>| {
        format!(
            "flex h-10 rounded-md border border-input bg-background mt-2 px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-23 {}",
            if state.current().is_none() { "red-border" } else { "" }
        )
    };
    render!(
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
                                class: "{inp_style(start_time)}",
                                id: "start-time",
                                placeholder: "08",
                                r#type: "time",

                                oninput: move |evt| start_time.set(parse_time(&evt.value))
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
                                class: "{inp_style(end_time)}",
                                id: "end-time",
                                placeholder: "17",
                                r#type: "time",
                                oninput: move |evt| {
                                    end_time.set(parse_time(&evt.value));
                                    console_log!("End time is now: {:?}", evt.value);
                                }
                            }
                        }
                    }
                }
                button {
                    class: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-white hover:bg-primary/90 h-10 px-4 py-2 w-full",
                    onclick: move |_| {
                        let (Some(start_time), Some(end_time)) = (*start_time.current(), *end_time.current()) else {
                            return;
                        };
                        let config = config::Config {
                            block_time_start: start_time,
                            block_time_end: end_time,
                        };
                        console_log!("{:?}", config::set_configs(&config));
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

fn show_settings(cx: Scope, config: config::Config) -> Element {
    // Idea for this page:
    // 1. Show the current settings (i.e start and end time)
    // 2. Allow the user to change the settings by reverting to the previous page.
    // 3. Show some statistics (hours of YouTube accessed today, etc.)
    console_log!("from show_settings: {config:?}");
    render!( h1 { "Speaking from show_settings!" } )
}
