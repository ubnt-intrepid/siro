use siro::{event::on_input, html, prelude::*, vdom::CustomNode, App, Mailbox, VNode};
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

fn view<M: ?Sized>(model: &Model, mailbox: &M) -> impl Into<VNode>
where
    M: Mailbox<Msg = Msg>,
{
    html::div() //
        .id("editor")
        .child(
            html::textarea() //
                .event(mailbox, on_input(Msg::Edit)),
        )
        .child(markdown_preview(&model.input))
}

fn markdown_preview(input: &str) -> impl Into<VNode> {
    use pulldown_cmark::{Options, Parser};

    let parser = Parser::new_ext(
        input,
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES,
    );

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    let mut sanitizer = ammonia::Builder::new();
    sanitizer.add_allowed_classes("code", &["language-rust"]);
    output = sanitizer.clean(&output).to_string();

    CustomNode::new(move |document| {
        let node = document.create_element("div")?;
        node.set_inner_html(&output);
        Ok(node.into())
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
