use siro::{html, prelude::*, App, Mailbox, VNode};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

mod counter {
    use siro::{html, prelude::*, vdom::VNode, Mailbox};

    #[derive(Default, Clone)]
    pub struct Model {
        value: i32,
    }

    pub enum Msg {
        Increment,
        Decrement,
        Reset,
    }

    pub fn update(model: &mut Model, msg: Msg) {
        match msg {
            Msg::Increment => model.value += 1,
            Msg::Decrement => model.value -= 1,
            Msg::Reset => model.value = 0,
        }
    }

    pub fn view(model: &Model, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
        html::div().children((
            html::button() //
                .on("click", mailbox, |_| Msg::Decrement)
                .child("-"),
            " ",
            model.value.to_string(),
            " ",
            html::button() //
                .on("click", mailbox, |_| Msg::Increment)
                .child("+"),
            " ",
            html::button() //
                .on("click", mailbox, |_| Msg::Reset)
                .child("Reset"),
        ))
    }
}

type Model = Vec<counter::Model>;

struct Msg(usize, counter::Msg);

fn update(model: &mut Model, msg: Msg) {
    let Msg(i, msg) = msg;
    counter::update(&mut model[i], msg);
}

fn view(model: &Model, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
    html::div().append(model.iter().enumerate().map(|(i, m)| {
        html::div() //
            .child(format!("{}: ", i))
            .child(counter::view(m, &mailbox.map(move |msg| Msg(i, msg))))
    }))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;

    let mut model = vec![counter::Model::default(); 10];
    app.render(view(&model, &app))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model, &app))?;
    }

    Ok(())
}
