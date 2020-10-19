use siro::prelude::*;
use siro_html as html;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Default)]
struct Model {
    value: i32,
}

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

fn view(model: &Model) -> impl Node<Msg = Msg> {
    html::div(
        (),
        (
            html::button(html::event::on_click(|| Msg::Decrement), "-"),
            " ",
            model.value.to_string(),
            " ",
            html::button(html::event::on_click(|| Msg::Increment), "+"),
            " ",
            html::button(html::event::on_click(|| Msg::Reset), "Reset"),
        ),
    )
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::mount("#app")?;

    let mut model = Model { value: 0 };
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
