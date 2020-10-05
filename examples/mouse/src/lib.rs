use siro::{prelude::*, svg, view::attribute, App, View};
use wasm_bindgen::{prelude::*, JsCast as _};
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

fn view(model: &Model) -> impl View<Msg = Msg> {
    // FIXME: avoid using .attribute()
    svg::svg((
        attribute("viewbox", "-500 -500 1000 1000"),
        attribute("width", "100%"),
        attribute("height", "100%"),
        attribute("style", "position: fixed; top: 0px; left: 0px;"),
        svg::circle((
            svg::r(20),
            svg::cx(model.x),
            svg::cy(model.y),
            svg::fill(if model.clicked { "red" } else { "#ad7fa8" }),
        )),
    ))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;

    let _mousedown = siro::subscription::window_event("mousedown")
        .map(|event| Msg::MouseDown(event.clone().unchecked_into()))
        .subscribe(&app)?;

    let _mousemove = siro::subscription::window_event("mousemove")
        .map(|event| Msg::MouseMove(event.clone().unchecked_into()))
        .subscribe(&app)?;

    let _mouseup = siro::subscription::window_event("mouseup")
        .map(|event| Msg::MouseUp(event.clone().unchecked_into()))
        .subscribe(&app)?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg)?;
        app.render(view(&model))?;
    }

    Ok(())
}
