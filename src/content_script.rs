use crate::config::{self, Config, ConfigBuilder, ConfigError};
use crate::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub async fn touch_grass() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let config = match config::get_config().await {
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

    record_watch_time(&window)
        .await
        .expect("Couldn't start recording watch statistics");
    // CAREFUL! If start_time > end_time (eg: start_time=10:00PM, end_time=6:00AM)
    //  then it isn't a simple range-check.
    //  Either I:
    //  - check it in blocks of two, i.e (end_time..24*60) or (0..start_time) for this case and another
    //    if block for normal case where start_time <= end_time.
    //  - or I check if it is NOT in the range (smaller_time..larger_time).
    //  For some reason, I like the second one better, as it can be composed without multiple if-else's,
    //  as done below.

    let normal_check = config.block_time_start <= config.block_time_end;
    let early_hr = config.block_time_start.min(config.block_time_end);
    let late_hr = config.block_time_start.max(config.block_time_end);
    if normal_check ^ (early_hr..late_hr).contains(&curr_time) {
        // Exit out of this script. Let the page load normally.
        return;
    }

    remove_distractions(&document);
}

/**
 * Currently I've decided to update the watch time every minute.
 * Of course this means the watch time is always off by a maximum of 1 minute.
 * Another alternative would be to have a `start_recording()` function
 * that just simply logs in the current time, and a `stop_recording()` function
 * that is somehow called when the page is closed (maybe some "run_at" attribute in manifest.json).
 * The reason I've decided not to do that:
 * - I'd need to add yet another JS binding file because ManifestV2 can't directly
 * call into .wasm (afaik), and needs a glue JS file; which will then call into a disjointed Rust file,
 * which would need to be included in the module (for LSP to work).
 * but wouldn't actually be related to the functioning of the module. This incrementally
 * makes the project feel more confusing when files are grouped together not by functionality
 * but by nature of language.
 */
async fn record_watch_time(document: &web_sys::Window) -> Result<(), ConfigError> {
    config::update_config(ConfigBuilder {
        total_usage: Some(1),
        ..Default::default()
    })
    .await?;

    let closure = Closure::<dyn Fn()>::new(|| spawn_local(increment_total_usage()));

    // TODO: Impl a global error enum and refactor ConfigError to be that Error type.
    // And yes then create one for this situation.
    document
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1 * 60 * 1000,
        )
        .expect("Failed to setInterval the total usage tracker.");

    closure.forget();
    Ok(())
}

async fn increment_total_usage() {
    let old_config = config::get_config().await.unwrap_or(Config::default());
    let new_config = Config {
        total_usage: old_config.total_usage + 1,
        ..old_config
    };
    // WHY .unwrap(): I already have meaningful messages for the errors that're going to be propagated.
    // No need to muddle it with a generic-ass message again.
    config::set_config(new_config).await.unwrap();
}

fn remove_distractions(document: &web_sys::Document) {
    let homepage = document.get_elements_by_tag_name("ytd-rich-grid-renderer");
    let sidebar = document.get_elements_by_tag_name("ytd-watch-next-secondary-results-renderer");

    let distractions = [homepage, sidebar];

    distractions.into_iter().for_each(|distraction| {
        distraction
            .item(0)
            .map(|el| el.set_inner_html("<h1>🌱\nPADHLE</h1>"));
    });
}
