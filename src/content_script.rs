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

    // CAREFUL! If start_time > end_time (eg: start_time=10:00PM, end_time=6:00AM)
    //  then it isn't a simple range-check.
    //  Either I:
    //  - check it in blocks of two, i.e (end_time..24*60) or (0..start_time) for this case and another
    //    if block for normal case where start_time <= end_time.
    //  - or I check if it is NOT in the range (start_time..end_time).
    //  For some reason, I like the second one better, as it can be composed without multiple if-else's,
    //  as done below.

    let normal_check = config.block_time_start <= config.block_time_end;
    let early_hr = config.block_time_start.min(config.block_time_end);
    let late_hr = config.block_time_start.max(config.block_time_end);
    if normal_check ^ (early_hr..late_hr).contains(&curr_time) {
        // Exit out of this script. Let the page load normally.
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
