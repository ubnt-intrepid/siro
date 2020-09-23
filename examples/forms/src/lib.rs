use siro::{
    builder::{html, ElementBuilder as _},
    vdom::Node,
    App, Mailbox,
};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Debug, Default)]
struct Model {
    name: String,
    password: String,
    password_again: String,
}

#[derive(Debug)]
enum Msg {
    Name(String),
    Password(String),
    PasswordAgain(String),
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Name(name) => model.name = name,
        Msg::Password(password) => model.password = password,
        Msg::PasswordAgain(password) => model.password_again = password,
    }
}

fn view(model: &Model, mailbox: &(impl Mailbox<Msg> + 'static)) -> impl Into<Node> {
    html::div()
        .child(
            html::input::text()
                .placeholder("Name")
                .value(model.name.clone())
                .on_input(mailbox, Msg::Name),
        )
        .child(
            html::input::password()
                .placeholder("Password")
                .value(model.password.clone())
                .on_input(mailbox, Msg::Password),
        )
        .child(
            html::input::password()
                .placeholder("Re-enter Password")
                .value(model.password_again.clone())
                .on_input(mailbox, Msg::PasswordAgain),
        )
        .child(if model.password == model.password_again {
            html::div() //
                .class("text-green")
                .child("Ok")
        } else {
            html::div() //
                .class("text-red")
                .child("Password does not match!")
        })
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;
    let mailbox = app.mailbox();

    let mut model = Model::default();
    app.render(view(&model, &mailbox))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model, &mailbox))?;
    }

    Ok(())
}
