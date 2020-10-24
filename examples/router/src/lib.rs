use siro::prelude::*;
use siro_html as html;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

struct Model {
    url: String,
}

enum Msg {
    Navigate(String),
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Navigate(url) => model.url = url,
    }
}

#[rustfmt::skip]
fn view(model: &Model) -> impl Node<Msg = Msg> {
    const URLS: &[&str] = &[
        "#/",
        "#/foo",
        "#/foo/bar",
        "#/foo/bar?baz=quux",
        "#/foo/bar?baz#quux",
    ];

    html::div((), (
        html::ul((), siro::children::iter(URLS.into_iter().map(|&url| {
            html::li((),
                html::a(
                    ( html::attr::href(url), siro::attr::attribute("is-router-link", true) ),
                    url,
                ),
            )
        }))),
        html::p((), ("Current URL: ", model.url.clone())),
    ))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::mount("#app")?;

    let current_url = app.with_navigation(Msg::Navigate)?;

    let mut model = Model { url: current_url };
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
