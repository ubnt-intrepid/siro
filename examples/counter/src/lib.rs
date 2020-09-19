use siro::{vdom, Mailbox};
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

fn view(model: &Model, mailbox: &Mailbox<Msg>) -> impl Into<vdom::Node> {
    vdom::element("div")
        .child(
            vdom::element("button") //
                .listener(mailbox.on("click", |_| Msg::Decrement))
                .child("-"),
        )
        .child(" ")
        .child(model.value.to_string())
        .child(" ")
        .child(
            vdom::element("button") //
                .listener(mailbox.on("click", |_| Msg::Increment))
                .child("+"),
        )
        .child(" ")
        .child(
            vdom::element("button") //
                .listener(mailbox.on("click", |_| Msg::Reset))
                .child("Reset"),
        )
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let ctx = siro::global_context() //
        .ok_or("cannot create global context")?;

    let mountpoint = ctx
        .select("#app") //
        .ok_or("cannot find `#app` in document")?;
    siro::util::remove_children(&mountpoint)?;

    let mut app = ctx.mount(mountpoint.as_ref())?;
    let (mailbox, mut mails) = siro::mailbox();

    let mut model = Model { value: 0 };
    app.render(&ctx, view(&model, &mailbox))?;

    while let Some(msg) = mails.recv().await {
        update(&mut model, msg);
        app.render(&ctx, view(&model, &mailbox))?;
    }

    Ok(())
}
