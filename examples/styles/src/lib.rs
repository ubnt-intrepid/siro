use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::<()>::mount("#app")?;

    app.render({
        use siro::attr::style;
        use siro_html::{div, span};

        div(
            (),
            (
                span(style("color", "red"), "Hello"),
                ", from ",
                span(
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
