use meow::{vdom, Meow};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_timer::Delay;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let meow = Meow::init()?;

    let node = meow
        .select("#app")
        .ok_or("cannot find `#app` in document")?;

    let mut app = meow.mount(
        &node,
        vdom::element("div") //
            .child("Hello"),
    )?;

    Delay::new(Duration::from_secs(3)).await.unwrap_throw();

    app.draw(
        &meow, //
        vdom::element("div") //
            .child("Hello, from ")
            .child(
                vdom::element("strong") //
                    .child("Rust!"),
            ),
    )?;

    Ok(())
}
