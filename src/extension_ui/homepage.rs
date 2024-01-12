use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_app() {
    console_error_panic_hook::set_once();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let counter = cx.use_hook(|| 0);
    render!(
        h1 {
            "hello world! The counter value is {counter}"
        }
        button {
            onclick: move |_| {
                *counter += 1;
                cx.needs_update();
            },
            "Click me daddy 😩"
        }
    )
}
