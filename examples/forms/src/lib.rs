use siro::{vdom, Mailbox};
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

fn view(model: &Model, mailbox: &Mailbox<Msg>) -> impl Into<vdom::Node> {
    fn target_value(e: &web_sys::Event) -> Option<String> {
        js_sys::Reflect::get(&&e.target()?, &"value".into())
            .ok()?
            .as_string()
    }

    vdom::html("div")
        .child(
            vdom::html("input")
                .attribute("type", "text")
                .attribute("placeholder", "Name")
                .property("value", model.name.clone())
                .listener(mailbox.on("input", |e| Msg::Name(target_value(e).unwrap_or_default()))),
        )
        .child(
            vdom::html("input")
                .attribute("type", "password")
                .attribute("placeholder", "Password")
                .property("value", model.password.clone())
                .listener(mailbox.on("input", |e| {
                    Msg::Password(target_value(e).unwrap_or_default())
                })),
        )
        .child(
            vdom::html("input")
                .attribute("type", "password")
                .attribute("placeholder", "Re-enter Password")
                .property("value", model.password_again.clone())
                .listener(mailbox.on("input", |e| {
                    Msg::PasswordAgain(target_value(e).unwrap_or_default())
                })),
        )
        .child(if model.password == model.password_again {
            vdom::html("div")
                .attribute("class", "text-green")
                .child("Ok")
        } else {
            vdom::html("div")
                .attribute("class", "text-red")
                .child("Password does not match!")
        })
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

    let mut model = Model::default();
    app.render(&ctx, view(&model, &mailbox))?;

    while let Some(msg) = mails.recv().await {
        update(&mut model, msg);
        app.render(&ctx, view(&model, &mailbox))?;
    }

    Ok(())
}
