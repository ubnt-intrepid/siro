use siro::{svg, App};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = App::<()>::mount("#app")?;

    app.render({
        svg::svg(
            (
                svg::viewbox(0, 0, 400, 400),
                svg::width(400),
                svg::height(400),
            ),
            (
                svg::circle(
                    (
                        svg::cx(50),
                        svg::cy(50),
                        svg::r(40),
                        svg::fill("red"),
                        svg::stroke("black"),
                        svg::stroke_width(3),
                    ),
                    (),
                ),
                svg::rect(
                    (
                        svg::x(100),
                        svg::y(10),
                        svg::width(40),
                        svg::height(40),
                        svg::fill("green"),
                        svg::stroke("black"),
                        svg::stroke_width(2),
                    ),
                    (),
                ),
                svg::line(
                    (
                        svg::x1(20),
                        svg::y1(200),
                        svg::x2(200),
                        svg::y2(20),
                        svg::stroke("blue"),
                        svg::stroke_width(10),
                        svg::stroke_linecap("round"),
                    ),
                    (),
                ),
                svg::polyline(
                    (
                        svg::points(vec![
                            (200, 40),
                            (240, 40),
                            (240, 80),
                            (280, 80),
                            (280, 120),
                            (320, 120),
                            (320, 160),
                        ]),
                        svg::fill("none"),
                        svg::stroke("red"),
                        svg::stroke_width(4),
                        svg::stroke_dasharray(vec![20, 2]),
                    ),
                    (),
                ),
                svg::text(
                    (
                        svg::x(130),
                        svg::y(130),
                        svg::fill("black"),
                        svg::text_anchor("middle"),
                        svg::dominant_baseline("central"),
                        svg::transform("rotate(-45 130,130)"),
                    ),
                    "Welcome to Shape Club",
                ),
            ),
        )
    })?;

    Ok(())
}
