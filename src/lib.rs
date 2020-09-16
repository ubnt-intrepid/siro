use meow::{vdom::NodeFactory, Meow};
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

    let n = NodeFactory::default();

    let mut scene = meow.scene(
        &app,
        n.element("div") //
            .child(n.text("Hello")),
    )?;

    Delay::new(Duration::from_secs(3)).await.unwrap_throw();

    scene.set_view(
        &meow, //
        n.element("div") //
            .child(n.text("Hello, from "))
            .child(n.element("strong").child(n.text("Rust!"))),
    )?;

    Ok(())
}
