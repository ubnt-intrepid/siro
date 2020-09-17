use meow::{vdom, Meow};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_timer::Delay;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let meow = Meow::init()?;

    let app = meow
        .select("#app")
        .ok_or("cannot find `#app` in document")?;

    let mut scene = meow.scene(
        &app,
        vdom::element("div") //
            .child(vdom::text("Hello")),
    )?;

    Delay::new(Duration::from_secs(3)).await.unwrap_throw();

    scene.draw(
        &meow, //
        vdom::element("div") //
            .child(vdom::text("Hello, from "))
            .child(
                vdom::element("strong") //
                    .child(vdom::text("Rust!")),
            ),
    )?;

    Ok(())
}
