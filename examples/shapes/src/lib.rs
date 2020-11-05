use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> siro_web::Result<()> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount::<()>("#app")?;

    app.render({
        use siro::svg::{self, attr, svg};

        svg(
            (
                attr::view_box("0 0 400 400"),
                attr::width("400"),
                attr::height("400"),
            ),
            (
                svg::circle(
                    (
                        attr::cx("50"),
                        attr::cy("50"),
                        attr::r("40"),
                        attr::fill("red"),
                        attr::stroke("black"),
                        attr::stroke_width("3"),
                    ),
                    (),
                ),
                svg::rect(
                    (
                        attr::x("100"),
                        attr::y("10"),
                        attr::width("40"),
                        attr::height("40"),
                        attr::fill("green"),
                        attr::stroke("black"),
                        attr::stroke_width("2"),
                    ),
                    (),
                ),
                svg::line(
                    (
                        attr::x1("20"),
                        attr::y1("200"),
                        attr::x2("200"),
                        attr::y2("20"),
                        attr::stroke("blue"),
                        attr::stroke_width("10"),
                        attr::stroke_linecap("round"),
                    ),
                    (),
                ),
                svg::polyline(
                    (
                        attr::points(
                            "200,40 \
                             240,40 \
                             240,80 \
                             280,80 \
                             280,120 \
                             320,120 \
                             320,160",
                        ),
                        attr::fill("none"),
                        attr::stroke("red"),
                        attr::stroke_width("4"),
                        attr::stroke_dasharray("20,2"),
                    ),
                    (),
                ),
                svg::text(
                    (
                        attr::x("130"),
                        attr::y("130"),
                        attr::fill("black"),
                        attr::text_anchor("middle"),
                        attr::dominant_baseline("central"),
                        attr::transform("rotate(-45 130,130)"),
                    ),
                    "Welcome to Shape Club",
                ),
            ),
        )
    })?;

    Ok(())
}
