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

fn view(model: &Model, mailbox: &Mailbox<Msg>) -> impl Into<Node> {
    fn target_value(e: &web_sys::Event) -> Option<String> {
        js_sys::Reflect::get(&&e.target()?, &"value".into())
            .ok()?
            .as_string()
    }

    html::div().children((
        html::input::text()
            .placeholder("Name")
            .value(model.name.clone())
            .listener(mailbox.on("input", |e| Msg::Name(target_value(e).unwrap_or_default()))),
        html::input::password()
            .placeholder("Password")
            .value(model.password.clone())
            .listener(mailbox.on("input", |e| {
                Msg::Password(target_value(e).unwrap_or_default())
            })),
        html::input::password()
            .placeholder("Re-enter Password")
            .value(model.password_again.clone())
            .listener(mailbox.on("input", |e| {
                Msg::PasswordAgain(target_value(e).unwrap_or_default())
            })),
        if model.password == model.password_again {
            html::div() //
                .class("text-green")
                .child("Ok")
        } else {
            html::div() //
                .class("text-red")
                .child("Password does not match!")
        },
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

    let mut model = Model::default();
    app.render(view(&model, &mailbox))?;

    while let Some(msg) = mails.recv().await {
        update(&mut model, msg);
        app.render(view(&model, &mailbox))?;
    }

    Ok(())
}
