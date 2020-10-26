use siro::prelude::*;
use siro_web::subscription::window_event;

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Default)]
struct Model {
    x: i32,
    y: i32,
    clicked: bool,
}

struct Msg {
    event: MouseEvent,
    button: Option<Button>,
}

#[derive(Deserialize)]
struct MouseEvent {
    #[serde(rename = "clientX")]
    client_x: i32,
    #[serde(rename = "clientY")]
    client_y: i32,
}

enum Button {
    Up,
    Down,
}

fn update(model: &mut Model, Msg { event, button }: Msg) -> Result<(), JsValue> {
    model.x = event.client_x;
    model.y = event.client_y;

    match button {
        Some(Button::Down) => model.clicked = true,
        Some(Button::Up) => model.clicked = false,
        _ => (),
    }

    Ok(())
}

fn view(model: &Model) -> impl Nodes<Msg> {
    use siro::{
        svg::{
            attr::{cx, cy, fill, height, r, viewbox, width},
            circle, svg,
        },
        vdom::style,
    };

    svg(
        (
            viewbox("-500 -500 1000 1000"),
            width("100%"),
            height("100%"),
            style("position", "fixed"),
            style("top", "0px"),
            style("left", "0px"),
        ),
        circle(
            (
                r("20"),
                cx(model.x.to_string()),
                cy(model.y.to_string()),
                fill(if model.clicked { "red" } else { "#ad7fa8" }),
            ),
            (),
        ),
    )
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::mount("#app")?;

    let _mousedown = app.subscribe(window_event("mousedown").map(|event| Msg {
        event,
        button: Some(Button::Down),
    }))?;
    let _mousemove = app.subscribe(window_event("mousemove").map(|event| Msg {
        event,
        button: None,
    }))?;
    let _mouseup = app.subscribe(window_event("mouseup").map(|event| Msg {
        event,
        button: Some(Button::Up),
    }))?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg)?;
        app.render(view(&model))?;
    }

    Ok(())
}
