use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> siro_web::Result<()> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount::<()>("#app")?;

    app.render({
        use siro::{
            html::{div, span},
            vdom::style,
        };

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
