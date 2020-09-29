use siro::{prelude::*, App};
use siro_svg as svg;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::<()>::mount("#app")?;

    app.render({
        svg::svg()
            .viewbox(0, 0, 400, 400)
            .width(400)
            .height(400)
            .children((
                svg::circle()
                    .cx(50)
                    .cy(50)
                    .r(40)
                    .fill("red")
                    .stroke("black")
                    .stroke_width(3),
                svg::rect()
                    .x(100)
                    .y(10)
                    .width(40)
                    .height(40)
                    .fill("green")
                    .stroke("black")
                    .stroke_width(2),
                svg::line()
                    .x1(20)
                    .y1(200)
                    .x2(200)
                    .y2(20)
                    .stroke("blue")
                    .stroke_width(10)
                    .stroke_linecap("round"),
                svg::polyline()
                    .points(vec![
                        (200, 40),
                        (240, 40),
                        (240, 80),
                        (280, 80),
                        (280, 120),
                        (320, 120),
                        (320, 160),
                    ])
                    .fill("none")
                    .stroke("red")
                    .stroke_width(4)
                    .stroke_dasharray(vec![20, 2]),
                svg::text()
                    .x(130)
                    .y(130)
                    .fill("black")
                    .text_anchor("middle")
                    .dominant_baseline("central")
                    .transform("rotate(-45 130,130)")
                    .child("Welcome to Shape Club"),
            ))
    })?;

    Ok(())
}
