use siro::{prelude::*, App, Mailbox, VNode};
use siro_html::{self as html, input::on_input};
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

fn view(model: &Model, mailbox: &(impl Mailbox<Msg = Msg> + 'static)) -> impl Into<VNode> {
    html::div()
        .child(
            html::input::text()
                .placeholder("Name")
                .value(model.name.clone())
                .event(mailbox, on_input(Msg::Name)),
        )
        .child(
            html::input::password()
                .placeholder("Password")
                .value(model.password.clone())
                .event(mailbox, on_input(Msg::Password)),
        )
        .child(
            html::input::password()
                .placeholder("Re-enter Password")
                .value(model.password_again.clone())
                .event(mailbox, on_input(Msg::PasswordAgain)),
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

    let mut model = Model::default();
    app.render(view(&model, &app))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model, &app))?;
    }

    Ok(())
}
