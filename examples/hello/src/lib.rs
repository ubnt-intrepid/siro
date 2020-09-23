use siro::{
    builder::{html, ElementBuilder as _},
    App,
};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Mount a Siro application on the specified mountpoint.
    let mut app = App::<()>::mount("#app")?;

    // Draw the virtual DOM.
    app.render({
        html::h1() //
            .child("Hello from Rust!")
    })?;

    Ok(())
}
