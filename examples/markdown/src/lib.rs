use siro::{attr::attribute, event, html, view::raw, App, View};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Debug, Default)]
struct Model {
    input: String,
}

#[derive(Debug)]
enum Msg {
    Edit(String),
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Edit(input) => model.input = input,
    }
}

fn view(model: &Model) -> impl View<Msg = Msg> {
    html::div(
        attribute("id", "editor"),
        (
            html::textarea(event::on_input(Msg::Edit), ()),
            raw(markdown_preview(&model.input)),
        ),
    )
}

fn markdown_preview(input: &str) -> String {
    use pulldown_cmark::{Options, Parser};

    let parser = Parser::new_ext(
        input,
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
    );

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    let mut sanitizer = ammonia::Builder::new();
    sanitizer.add_allowed_classes("code", &["language-rust"]);
    sanitizer.clean(&output).to_string()
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
