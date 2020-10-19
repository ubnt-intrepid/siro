use siro::prelude::*;
use siro_web::subscription::{window_event, WindowEvent};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Debug, Default)]
struct Model {
    x: i32,
    y: i32,
    clicked: bool,
}

#[derive(Debug)]
enum Msg {
    MouseMove(WindowEvent),
    MouseDown(WindowEvent),
    MouseUp(WindowEvent),
}

fn update(model: &mut Model, msg: Msg) -> Result<(), JsValue> {
    match msg {
        Msg::MouseMove(event) => {
            let event: &web_sys::MouseEvent = event.unchecked_ref();
            model.x = event.client_x();
            model.y = event.client_y();
        }
        Msg::MouseDown(event) => {
            let event: &web_sys::MouseEvent = event.unchecked_ref();
            model.x = event.client_x();
            model.y = event.client_y();
            model.clicked = true;
        }
        Msg::MouseUp(event) => {
            let event: &web_sys::MouseEvent = event.unchecked_ref();
            model.x = event.client_x();
            model.y = event.client_y();
            model.clicked = false;
        }
    }

    Ok(())
}

fn view(model: &Model) -> impl Node<Msg = Msg> {
    use siro::attr::style;
    use siro_svg::{
        attr::{cx, cy, fill, height, r, viewbox, width},
        circle, svg,
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

    let _mousedown = app.subscribe(window_event("mousedown").map(Msg::MouseDown))?;
    let _mousemove = app.subscribe(window_event("mousemove").map(Msg::MouseMove))?;
    let _mouseup = app.subscribe(window_event("mouseup").map(Msg::MouseUp))?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg)?;
        app.render(view(&model))?;
    }

    Ok(())
}
