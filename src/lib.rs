use meow::Meow;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_timer::Delay;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let meow = Meow::init()?;

    let mut scene = meow.scene(
        "#app", //
        "Hello",
    )?;

    Delay::new(Duration::from_secs(3)).await.unwrap_throw();

    scene.set_view(
        &meow, //
        "Hello, from Rust!",
    )?;

    Ok(())
}
