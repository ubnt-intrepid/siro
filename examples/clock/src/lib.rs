use siro::{prelude::*, svg, App, View};
use std::f32;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Debug)]
struct Model {
    date: js_sys::Date,
}

#[derive(Debug)]
enum Msg {
    Tick,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Tick => model.date = js_sys::Date::new_0(),
    }
}

fn view(model: &Model) -> impl View<Msg = Msg> {
    let hour = model.date.get_hours() % 12;
    let minute = model.date.get_minutes() % 60;
    let second = model.date.get_seconds() % 60;

    svg::svg((
        svg::viewbox(0, 0, 400, 400),
        svg::width(400),
        svg::height(400),
    ))
    .with((
        svg::circle((
            svg::cx(200),
            svg::cy(200),
            svg::r(120),
            svg::fill("#1293D8"),
        )),
        view_hand("white", 6, 60.0, hour as f32 / 12.0),
        view_hand("white", 6, 90.0, minute as f32 / 60.0),
        view_hand("#ff3860", 3, 90.0, second as f32 / 60.0),
        svg::text((
            svg::x(200),
            svg::y(260),
            svg::text_anchor("middle"),
            svg::dominant_baseline("central"),
            svg::fill("white"),
            format!("{:02}:{:02}:{:02}", hour, minute, second),
        )),
    ))
}

fn view_hand(stroke: &'static str, width: i32, length: f32, turns: f32) -> impl View<Msg = Msg> {
    let t = f32::consts::TAU * (turns - 0.25);
    let x = (200.0 + length * t.cos()) as i32;
    let y = (200.0 + length * t.sin()) as i32;

    svg::line((
        svg::x1(200),
        svg::y1(200),
        svg::x2(x),
        svg::y2(y),
        svg::stroke(stroke),
        svg::stroke_width(width),
        svg::stroke_linecap("round"),
    ))
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::mount("#app")?;

    let _guard = siro::subscription::animation_frames()
        .map(|_timestamp| Msg::Tick) //
        .subscribe(&app)?;

    let mut model = Model {
        date: js_sys::Date::new_0(),
    };
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
