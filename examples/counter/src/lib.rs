use siro::{
    builder::{html, ElementBuilder as _},
    vdom::Node,
    App, Mailbox,
};
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

fn view(model: &Model, mailbox: &Mailbox<Msg>) -> impl Into<Node> {
    html::div().children((
        html::button() //
            .listener(mailbox.on("click", |_| Msg::Decrement))
            .child("-"),
        " ",
        model.value.to_string(),
        " ",
        html::button() //
            .listener(mailbox.on("click", |_| Msg::Increment))
            .child("+"),
        " ",
        html::button() //
            .listener(mailbox.on("click", |_| Msg::Reset))
            .child("Reset"),
    ))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mountpoint = siro::util::select("#app") //
        .ok_or("cannot find `#app` in document")?;
    siro::util::remove_children(&mountpoint)?;

    let mut app = App::mount(mountpoint.as_ref())?;
    let (mailbox, mut mails) = siro::mailbox();

    let mut model = Model { value: 0 };
    app.render(view(&model, &mailbox))?;

    while let Some(msg) = mails.recv().await {
        update(&mut model, msg);
        app.render(view(&model, &mailbox))?;
    }

    Ok(())
}
