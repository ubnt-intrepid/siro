use siro::prelude::*;
use siro::App;

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
    MouseMove(web_sys::MouseEvent),
    MouseDown(web_sys::MouseEvent),
    MouseUp(web_sys::MouseEvent),
}

fn update(model: &mut Model, msg: Msg) -> Result<(), JsValue> {
    match msg {
        Msg::MouseMove(event) => {
            model.x = event.client_x();
            model.y = event.client_y();
        }
        Msg::MouseDown(event) => {
            model.x = event.client_x();
            model.y = event.client_y();
            model.clicked = true;
        }
        Msg::MouseUp(event) => {
            model.x = event.client_x();
            model.y = event.client_y();
            model.clicked = false;
        }
    }

    Ok(())
}

fn view(model: &Model) -> impl Node<Msg = Msg> {
    use siro::{
        attr::style,
        svg::{
            attr::{cx, cy, fill, height, r, viewbox, width},
            circle, svg,
        },
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

    let mountpoint = siro::util::select("#app").ok_or("missing #app")?;
    let mut app = App::mount(mountpoint)?;

    let _mousedown = app.subscribe(
        siro::subscription::window_event("mousedown")
            .map(|event| Msg::MouseDown(event.unchecked_into())),
    )?;
    let _mousemove = app.subscribe(
        siro::subscription::window_event("mousemove")
            .map(|event| Msg::MouseMove(event.unchecked_into())),
    )?;
    let _mouseup = app.subscribe(
        siro::subscription::window_event("mouseup")
            .map(|event| Msg::MouseUp(event.unchecked_into())),
    )?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg)?;
        app.render(view(&model))?;
    }

    Ok(())
}
