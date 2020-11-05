use siro::prelude::*;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

#[derive(Default)]
struct Model {
    input: String,
}

// ==== update ====

enum Msg {
    Edit(String),
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Edit(input) => model.input = input,
    }
}

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> {
    use siro::html::{attr::id, div, event::on_input, textarea};

    div(
        id("editor"),
        (
            textarea(on_input(Msg::Edit), ()),
            view_markdown_preview(&model.input),
        ),
    )
}

fn view_markdown_preview(input: &str) -> impl Nodes<Msg> {
    use pulldown_cmark::{Options, Parser};
    use siro::{html::div, vdom::inner_html};

    let parser = Parser::new_ext(
        input,
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
    );

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    let mut sanitizer = ammonia::Builder::new();
    sanitizer.add_allowed_classes("code", &["language-rust"]);
    output = sanitizer.clean(&output).to_string();

    div(inner_html(output), ())
}

// ==== runtime ====

#[wasm_bindgen(start)]
pub async fn main() -> siro_web::Result<()> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount_to_body()?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
