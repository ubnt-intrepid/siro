use siro::vdom;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Instantiate the Siro global context.
    let ctx = siro::global_context() //
        .ok_or("cannot create global context")?;

    // Find the mountpoint node for the application in the DOM.
    let mountpoint = ctx
        .select("#app") //
        .ok_or("cannot find `#app` in document")?;

    // Remove all childlen from mountpoint node.
    siro::util::remove_children(&mountpoint)?;

    // Mount a Siro application on the specified mountpoint.
    let mut app = ctx.mount(mountpoint.as_ref())?;

    // Draw the virtual DOM.
    app.render(&ctx, {
        vdom::html("h1") //
            .child("Hello from Rust!")
    })?;

    Ok(())
}
