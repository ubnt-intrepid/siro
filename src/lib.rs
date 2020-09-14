use draco::{Application, VNode};
use wasm_bindgen::prelude::*;

struct MyApp;

impl Application for MyApp {
    type Message = ();

    fn view(&self) -> VNode<Self::Message> {
        "Hello from Rust!".into()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window
        .document()
        .ok_or("should have a document on window")?;

    let app = document
        .get_element_by_id("app")
        .ok_or("missing `app` in document")?;

    let node = document.create_element("div")?;
    app.append_child(&node)?;

    let _mailbox = draco::start(MyApp, node.into());

    Ok(())
}
