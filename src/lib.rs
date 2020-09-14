use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window
        .document()
        .ok_or("should have a document on window")?;

    let app = document
        .get_element_by_id("app")
        .ok_or("missing `app` in document")?;

    app.set_inner_html("Hello from Rust!");

    Ok(())
}
