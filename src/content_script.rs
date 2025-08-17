use crate::config::{self, Config, StorageError};
use crate::console_log;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub async fn touch_grass() {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let storage = match config::get_storage().await {
        Ok(storage) => storage,
        Err(e @ StorageError::EmptyStorage) => {
            console_log!("Not touching grass because no time slot set: {e}");
            return;
        }
        Err(e) => {
            console_log!("Error while getting storage: {e}");
            return;
        }
    };
    console_log!("Storage: {storage:?}");

    let curr_time = js_sys::Date::new_0();
    if !(within_active_time_window(&storage.user_config, &curr_time)
        && within_active_day_window(&storage.user_config, &curr_time))
    {
        console_log!("Not within the active window, returning..");
        return;
    }

    record_watch_time(&window)
        .await
        .expect("Couldn't start recording watch statistics");

    remove_distractions(&document);
}

fn within_active_time_window(config: &Config, curr_time: &js_sys::Date) -> bool {
    let curr_time = curr_time.get_hours() * 60 + curr_time.get_minutes();
    console_log!("Curr time: {curr_time}");
    // CAREFUL! If start_time > end_time (eg: start_time=10:00PM, end_time=6:00AM)
    //  then it isn't a simple range-check.
    //  Either I:
    //  - handle both cases separately, i.e case start_time > end_time: if (end_time..24*60) || (0..start_time) and
    //    case start_time <= end_time: another if block for normal case where start_time <= end_time.
    //  - handle it in one go: chec if time is NOT in the range (smaller_time..larger_time).
    //  For some reason, I like the second one better, as it can be composed without multiple if-else's,
    //  as done below.
    let is_normal_check = config.block_time_start <= config.block_time_end;
    let early_hr = config.block_time_start.min(config.block_time_end);
    let late_hr = config.block_time_start.max(config.block_time_end);
    let outside_time_window = is_normal_check ^ (early_hr..late_hr).contains(&curr_time);

    return !outside_time_window;
}

fn within_active_day_window(config: &Config, curr_time: &js_sys::Date) -> bool {
    // Sunday is zero. Fuck that, why does it start with the weekend?
    let curr_day = curr_time.get_day();
    const NUM_DAYS_IN_WEEK: u32 = 7;
    let curr_day = (curr_day + NUM_DAYS_IN_WEEK - 1) % NUM_DAYS_IN_WEEK;

    (config.active_days & (1 << curr_day)) != 0
}

/**
 * Currently I've decided to update the watch time every minute.
 * Of course this means the watch time is always off by a maximum of 1 minute.
 * Another alternative would be to have a `start_recording` function
 * that just simply logs in the current time, and a `stop_recording()` function
 * that is somehow called when the page is closed (maybe some "run_at" attribute in manifest.json).
 * The reason I've decided not to do that:
 * - I'd need to add yet another JS binding file because ManifestV2 can't directly
 * call into .wasm (afaik) and needs a glue JS file; which will then call into a disjointed Rust file,
 * which would need to be included in the module (for LSP to work).
 * but wouldn't actually be related to the functioning of the module. This incrementally
 * makes the project feel more confusing when files are grouped together not by functionality
 * but by nature of language.
 * An alternative that I've not explored is if there's a way to do it at runtime (not describing it in manifest.json)
 * but rather programmtically via Rust code only. Then, I can bypass JS glue code.
 */
async fn record_watch_time(document: &web_sys::Window) -> Result<(), StorageError> {
    config::update_storage(|storage| {
        storage.total_usage = 1;
    })
    .await?;

    let closure = Closure::<dyn Fn()>::new(|| spawn_local(increment_total_usage()));

    document
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            1 * 60 * 1000,
        )
        .expect("Failed to setInterval the total usage tracker.");

    // Leaking memory here is fine, because this closure is supposed to live until the end of the page
    // anyways, as it belongs into a setInterval function.
    closure.forget();
    Ok(())
}

async fn increment_total_usage() {
    // WHY .unwrap(): I already have meaningful messages for the errors that're going to be propagated.
    // No need to muddle it with a generic-ass message again.
    config::update_storage(|storage| {
        storage.total_usage += 1;
    })
    .await
    .unwrap();
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
