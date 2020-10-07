use siro::{attr::style, html, App};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::<()>::mount("#app")?;

    app.render({
        html::div(
            (),
            (
                html::span(style("color", "red"), "Hello"),
                ", from ",
                html::span(
                    (
                        style("fontWeight", "bold"),
                        style("textDecoration", "underline"),
                    ),
                    "Rust",
                ),
                "!",
            ),
        )
    })?;

    Ok(())
}
