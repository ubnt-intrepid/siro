use siro::{attr, event, html, util::if_else, App, View};
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
    html::div(
        (),
        (
            html::input(
                (
                    html::attr::placeholder("Name"),
                    html::attr::value(model.name.clone()),
                    event::on_input(Msg::Name),
                ),
                (),
            ),
            html::input::password(
                (
                    html::attr::placeholder("Password"),
                    html::attr::value(model.password.clone()),
                    event::on_input(Msg::Password),
                ),
                (),
            ),
            html::input::password(
                (
                    html::attr::placeholder("Re-enter Password"),
                    html::attr::value(model.password_again.clone()),
                    event::on_input(Msg::PasswordAgain),
                ),
                (),
            ),
            if_else(
                model.password == model.password_again,
                || html::div(attr::class("text-green"), "Ok"),
                || html::div(attr::class("text-red"), "Password does not match!"),
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
