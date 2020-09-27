use siro::{prelude::*, svg, App, VNode};
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

fn view(model: &Model) -> impl Into<VNode> {
    // FIXME: avoid using .attribute()
    svg::svg()
        .attribute("viewbox", "-500 -500 1000 1000")
        .attribute("width", "100%")
        .attribute("height", "100%")
        .attribute("style", "position: fixed; top: 0px; left: 0px;")
        .child(
            svg::circle() //
                .r(20)
                .cx(model.x)
                .cy(model.y)
                .fill(if model.clicked { "red" } else { "#ad7fa8" }),
        )
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;

    let _mousedown = siro::subscription::window_event("mousedown", |event| {
        Some(Msg::MouseDown(event.clone().unchecked_into()))
    })
    .subscribe(&app)?;

    let _mousemove = siro::subscription::window_event("mousemove", |event| {
        Some(Msg::MouseMove(event.clone().unchecked_into()))
    })
    .subscribe(&app)?;

    let _mouseup = siro::subscription::window_event("mouseup", |event| {
        Some(Msg::MouseUp(event.clone().unchecked_into()))
    })
    .subscribe(&app)?;

    let mut model = Model::default();
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg)?;
        app.render(view(&model))?;
    }

    Ok(())
}