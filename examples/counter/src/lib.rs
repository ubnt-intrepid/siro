use siro::prelude::*;
use siro::{html, App};
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

    let mountpoint = siro::util::select("#app").ok_or("missing #app")?;
    let mut app = App::mount(mountpoint)?;

    let mut model = Model { value: 0 };
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
