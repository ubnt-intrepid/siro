use siro::prelude::*;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

#[derive(Default)]
struct Model {
    name: String,
    password: String,
    password_again: String,
}

// ==== update ====

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

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> {
    use siro::{
        html::{
            attr::{placeholder, value},
            div,
            event::on_input,
            input,
        },
        vdom::style,
    };

    div(
        (),
        (
            input::text((
                placeholder("Name"),
                value(model.name.clone()),
                on_input(Msg::Name),
            )),
            input::password((
                placeholder("Password"),
                value(model.password.clone()),
                on_input(Msg::Password),
            )),
            input::password((
                placeholder("Re-enter Password"),
                value(model.password_again.clone()),
                on_input(Msg::PasswordAgain),
            )),
            if model.password == model.password_again {
                div(style("color", "green"), "Ok")
            } else {
                div(style("color", "red"), "Passwords do not match!")
            },
        ),
    )
}

// ==== runtime ====

#[wasm_bindgen(start)]
pub async fn main() -> siro_web::Result<()> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount("#app")?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
