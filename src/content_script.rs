use wasm_bindgen::prelude::*;

const YT_VIDEO_SECTION: &str = "primary";

#[wasm_bindgen]
pub fn touch_grass() {
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window");

    document
        .get_element_by_id(YT_VIDEO_SECTION)
        .expect("Couldn't find the `primary` div")
        .set_inner_html("<h1>🌱\nPADHLE</h1>");
}
