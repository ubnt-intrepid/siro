use siro::{prelude::*, App, Mailbox, VNode};
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

fn view<M: ?Sized>(model: &Model, mailbox: &M) -> impl Into<VNode>
where
    M: Mailbox<Msg = Msg>,
{
    html::div().children((
        html::button() //
            .event(mailbox, siro::event::on("click", |_| Msg::Decrement))
            .child("-"),
        " ",
        model.value.to_string(),
        " ",
        html::button() //
            .event(mailbox, siro::event::on("click", |_| Msg::Increment))
            .child("+"),
        " ",
        html::button() //
            .event(mailbox, siro::event::on("click", |_| Msg::Reset))
            .child("Reset"),
    ))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;

    let mut model = Model { value: 0 };
    app.render(view(&model, &app))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model, &app))?;
    }

    Ok(())
}
