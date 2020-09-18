use futures::prelude::*;
use meow::{vdom, Mailbox};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Default)]
struct Model {
    count: i32,
}

enum Msg {
    Increment,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Increment => model.count += 1,
    }
}

fn view(model: &Model, mailbox: &Mailbox<Msg>) -> impl Into<vdom::Node> {
    vdom::element("button") //
        .listener(mailbox.on_click(|_| Msg::Increment))
        .child(format!("{}", model.count))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let ctx = meow::global_context().ok_or("cannot create global context")?;

    let mountpoint = ctx.select("#app").ok_or("cannot find `#app` in document")?;
    meow::util::remove_children(&mountpoint)?;

    let mut app = ctx.mount(mountpoint.as_ref())?;
    let (mailbox, mut incomings) = Mailbox::pair();

    let mut model = Model { count: 0 };
    app.render(&ctx, view(&model, &mailbox))?;

    while let Some(msg) = incomings.next().await {
        update(&mut model, msg);
        app.render(&ctx, view(&model, &mailbox))?;
    }

    Ok(())
}
