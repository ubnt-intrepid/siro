use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    // Mount a Siro application on the specified mountpoint.
    let mut app = env.mount::<()>("#app")?;

    // Draw the view.
    app.render("Hello from Rust!")?;

    Ok(())
}
