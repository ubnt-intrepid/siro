use meow::{vdom, Meow};
use wasm_bindgen::prelude::*;

type Model = ();

fn view(_: &Model) -> impl Into<vdom::Node> {
    "Hello from Rust!"
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let meow = Meow::init()?;

    let mut app = meow.mount(
        "#app", //
        Model::default(),
        view,
    )?;

    app.render(&meow);

    Ok(())
}
