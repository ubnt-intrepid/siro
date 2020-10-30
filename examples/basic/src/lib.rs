use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Mount a Siro application on the specified mountpoint.
    let mut app = siro_web::App::<()>::new()?;
    app.mount("#app")?;

    // Draw the view.
    app.render("Hello from Rust!")?;

    Ok(())
}
