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

    // Find the mountpoint node for the application in the DOM.
    let mountpoint = siro::util::select("#app") //
        .ok_or("cannot find `#app` in document")?;

    // Remove all childlen from mountpoint node.
    siro::util::remove_children(&mountpoint)?;

    // Mount a Siro application on the specified mountpoint.
    let mut app = App::<()>::mount(mountpoint.as_ref())?;

    // Draw the virtual DOM.
    app.render({
        html::h1() //
            .child("Hello from Rust!")
    })?;

    Ok(())
}
