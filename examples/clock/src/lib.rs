use siro::{
    builder::svg::{self, prelude::*},
    vdom::Node,
    App, Mailbox,
};
use std::f32;
use wasm_bindgen::{prelude::*, JsCast as _};
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[derive(Debug)]
struct Model {
    date: js_sys::Date,
}

#[derive(Debug)]
enum Msg {
    Tick(js_sys::Date),
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Tick(date) => model.date = date,
    }
}

fn view(model: &Model, _: &(impl Mailbox<Msg> + 'static)) -> impl Into<Node> {
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

fn view_hand(stroke: &'static str, width: i32, length: f32, turns: f32) -> impl Into<Node> {
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
    let mailbox = app.mailbox();

    let (_cb, _id) = {
        let mailbox = mailbox.clone();
        let cb = Closure::wrap(Box::new(move || {
            mailbox.send_message(Msg::Tick(js_sys::Date::new_0()));
        }) as Box<dyn FnMut()>);
        let id = app
            .window()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                1000,
            )?;
        (cb, id)
    };

    let mut model = Model {
        date: js_sys::Date::new_0(),
    };
    app.render(view(&model, &mailbox))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model, &mailbox))?;
    }

    Ok(())
}
