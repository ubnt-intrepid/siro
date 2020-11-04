use siro::html::{button, div, event::on_click};
use siro::prelude::*;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

#[derive(Default)]
struct Model {
    value: i32,
}

// ==== update ====

enum Msg {
    Increment,
    Decrement,
    Reset,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Increment => model.value += 1,
        Msg::Decrement => model.value -= 1,
        Msg::Reset => model.value = 0,
    }
}

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> {
    div(
        (),
        (
            button(on_click(|| Msg::Decrement), "-"),
            " ",
            model.value.to_string(),
            " ",
            button(on_click(|| Msg::Increment), "+"),
            " ",
            button(on_click(|| Msg::Reset), "Reset"),
        ),
    )
}

// ==== runtime ====

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount("#app")?;

    let mut model = Model { value: 0 };
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
