use siro::{prelude::*, App, VNode};
use siro_svg as svg;
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

fn view(model: &Model) -> impl Into<VNode> {
    let hour = model.date.get_hours() % 12;
    let minute = model.date.get_minutes() % 60;
    let second = model.date.get_seconds() % 60;

    svg::svg()
        .viewbox(0, 0, 400, 400)
        .width(400)
        .height(400)
        .child(
            svg::circle() //
                .cx(200)
                .cy(200)
                .r(120)
                .fill("#1293D8"),
        )
        .child(view_hand("white", 6, 60.0, hour as f32 / 12.0))
        .child(view_hand("white", 6, 90.0, minute as f32 / 60.0))
        .child(view_hand("#ff3860", 3, 90.0, second as f32 / 60.0))
        .child(
            svg::text()
                .x(200)
                .y(260)
                .text_anchor("middle")
                .dominant_baseline("central")
                .fill("white")
                .child(format!("{:02}:{:02}:{:02}", hour, minute, second)),
        )
}

fn view_hand(stroke: &'static str, width: i32, length: f32, turns: f32) -> impl Into<VNode> {
    let t = f32::consts::TAU * (turns - 0.25);
    let x = (200.0 + length * t.cos()) as i32;
    let y = (200.0 + length * t.sin()) as i32;

    svg::line()
        .x1(200)
        .y1(200)
        .x2(x)
        .y2(y)
        .stroke(stroke)
        .stroke_width(width)
        .stroke_linecap("round")
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
