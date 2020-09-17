use meow::{vdom, Meow};
use std::time::Duration;
use wasm_bindgen::{prelude::*, JsCast as _};
use wasm_timer::Delay;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let meow = Meow::init()?;

    let node = meow
        .select("#app")
        .ok_or("cannot find `#app` in document")?;

    {
        let node = node.dyn_ref::<web_sys::Element>().unwrap_throw();
        while let Some(child) = node.first_element_child() {
            node.remove_child(&*child)?;
        }
    }

    let mut app = meow.mount(&node)?;

    Delay::new(Duration::from_secs(3)).await.unwrap_throw();

    loop {
        app.draw(&meow, {
            vdom::element("div") //
                .child("Hello")
        })?;

        Delay::new(Duration::from_secs(3)).await.unwrap_throw();

        app.draw(&meow, {
            vdom::element("div") //
                .child("Hello, from ")
                .child(
                    vdom::element("strong") //
                        .child("Rust!"),
                )
        })?;

        Delay::new(Duration::from_secs(3)).await.unwrap_throw();
    }
}
