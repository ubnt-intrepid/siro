use siro::prelude::*;
use siro::{svg, vdom::style};

use std::f32;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

struct Model {
    current: Time,
}

struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

// ==== update ====

enum Msg {
    Tick(Time),
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Tick(current) => model.current = current,
    }
}

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> {
    let hours = model.current.hours;
    let minutes = model.current.minutes;
    let seconds = model.current.seconds;

    let second_turns = (seconds as f32) / 60.0;
    let minute_turns = (minutes as f32 + second_turns) / 60.0;
    let hour_turns = ((hours % 12) as f32 + minute_turns) / 12.0;

    let color = if hours >= 12 { "#F0A048" } else { "#1293D8" };

    svg::svg(
        (
            svg::attr::view_box("0 0 400 400"),
            svg::attr::width("400"),
            svg::attr::height("400"),
        ),
        (
            svg::circle(
                (
                    svg::attr::cx("200"),
                    svg::attr::cy("200"),
                    svg::attr::r("120"),
                    svg::attr::fill(color),
                ),
                (),
            ),
            view_hand("white", 6, 60.0, hour_turns),
            view_hand("white", 6, 90.0, minute_turns),
            view_hand("#ff3860", 3, 90.0, second_turns),
            svg::text(
                (
                    svg::attr::x("200"),
                    svg::attr::y("340"),
                    svg::attr::text_anchor("middle"),
                    svg::attr::dominant_baseline("central"),
                    svg::attr::fill(color),
                    style("fontWeight", "bold"),
                ),
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
            ),
        ),
    )
}

fn view_hand(stroke: &'static str, width: i32, length: f32, turns: f32) -> impl Nodes<Msg> {
    let t = 2.0 * f32::consts::PI * (turns - 0.25);
    let x = 200.0 + length * t.cos();
    let y = 200.0 + length * t.sin();

    svg::line(
        (
            svg::attr::x1("200"),
            svg::attr::y1("200"),
            svg::attr::x2(format!("{:.3}", x)),
            svg::attr::y2(format!("{:.3}", y)),
            svg::attr::stroke(stroke),
            svg::attr::stroke_width(width.to_string()),
            svg::attr::stroke_linecap("round"),
        ),
        (),
    )
}

// ==== runtime ====

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::mount("#app")?;

    let _frames = app.subscribe(
        siro_web::subscription::animation_frames() //
            .map(|_timestamp| Msg::Tick(current_time())),
    )?;

    let mut model = Model {
        current: current_time(),
    };
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}

fn current_time() -> Time {
    let date = js_sys::Date::new_0();
    Time {
        hours: date.get_hours(),
        minutes: date.get_minutes(),
        seconds: date.get_seconds(),
    }
}
