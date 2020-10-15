use siro::{vdom::text, App};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Mount a Siro application on the specified mountpoint.
    let mountpoint = siro::util::select("#app").ok_or("missing #app")?;
    let mut app = App::<()>::mount(mountpoint)?;

    // Draw the view.
    app.render(text("Hello from Rust!"))?;

    Ok(())
}
