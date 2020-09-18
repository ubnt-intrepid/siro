use futures::prelude::*;
use meow::{vdom, Mailbox, Meow};
use wasm_bindgen::{prelude::*, JsCast as _};
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

    let meow = Meow::init()?;

    let node = meow
        .select("#app")
        .ok_or("cannot find `#app` in document")?;

    {
        let node = node.dyn_ref::<web_sys::Element>().unwrap_throw();
        while let Some(child) = node.first_element_child() {
            node.remove_child(&*child)?;
        }
    }

    let mut app = meow.mount(&node)?;

    let mut model = Model { count: 0 };

    let (mailbox, mut rx) = Mailbox::<Msg>::pair();

    app.draw(&meow, view(&model, &mailbox))?;

    while let Some(msg) = rx.next().await {
        update(&mut model, msg);

        app.draw(&meow, view(&model, &mailbox))?;
    }

    Ok(())
}
