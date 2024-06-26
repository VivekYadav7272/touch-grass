use crate::config;
use crate::console_log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn touch_grass() {
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window");

    let config = match config::get_configs().await {
        Ok(config) => config,
        Err(e) => {
            console_log!("Error: {e}");
            return;
        }
    };
    console_log!("Config: {config:?}");

    let curr_time = js_sys::Date::new_0();
    let curr_time = curr_time.get_hours() * 60 + curr_time.get_minutes();
    console_log!("Curr time: {curr_time}");

    if !(config.block_time_start..config.block_time_end).contains(&curr_time) {
        return;
    }

    let homepage = document.get_elements_by_tag_name("ytd-rich-grid-renderer");
    let sidebar = document.get_elements_by_tag_name("ytd-watch-next-secondary-results-renderer");

    let distractions = [homepage, sidebar];

    distractions.into_iter().for_each(|distraction| {
        distraction
            .item(0)
            .map(|el| el.set_inner_html("<h1>🌱\nPADHLE</h1>"));
    });
}
