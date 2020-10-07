use siro::{
    attr::{attribute, class, property},
    event, html,
    util::if_else,
    App, View,
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

fn view(model: &Model) -> impl View<Msg = Msg> {
    let type_ = |value: &'static str| attribute("type", value);
    let placeholder = |value: &'static str| attribute("placeholder", value);
    let value = |value: String| property("value", value);

    html::div(
        (),
        (
            html::input(
                (
                    type_("text"),
                    placeholder("Name"),
                    value(model.name.clone()),
                    event::on_input(Msg::Name),
                ),
                (),
            ),
            html::input(
                (
                    type_("password"),
                    placeholder("Password"),
                    value(model.password.clone()),
                    event::on_input(Msg::Password),
                ),
                (),
            ),
            html::input(
                (
                    type_("password"),
                    placeholder("Re-enter Password"),
                    value(model.password_again.clone()),
                    event::on_input(Msg::PasswordAgain),
                ),
                (),
            ),
            if_else(
                model.password == model.password_again,
                || html::div(class("text-green"), "Ok"),
                || html::div(class("text-red"), "Password does not match!"),
            ),
        ),
    )
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
