use crate::console_log;
use crate::storage::config;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn touch_grass() {
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window");

    let homepage = document.get_elements_by_tag_name("ytd-rich-grid-renderer");
    let sidebar = document.get_elements_by_tag_name("ytd-watch-next-secondary-results-renderer");

    let distractions = [homepage, sidebar];

    distractions.into_iter().for_each(|distraction| {
        distraction
            .item(0)
            .map(|el| el.set_inner_html("<h1>🌱\nPADHLE</h1>"));
    });

    let config = config::get_configs().unwrap();
    console_log!("Config: {config:?}");
}
