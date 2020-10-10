use siro::App;
use siro_svg as svg;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mountpoint = siro::util::select("#app").ok_or("missing #app")?;
    let mut app = App::<()>::mount(mountpoint)?;

    app.render({
        svg::svg(
            (
                svg::attr::viewbox("0 0 400 400"),
                svg::attr::width("400"),
                svg::attr::height("400"),
            ),
            (
                svg::circle(
                    (
                        svg::attr::cx("50"),
                        svg::attr::cy("50"),
                        svg::attr::r("40"),
                        svg::attr::fill("red"),
                        svg::attr::stroke("black"),
                        svg::attr::stroke_width("3"),
                    ),
                    (),
                ),
                svg::rect(
                    (
                        svg::attr::x("100"),
                        svg::attr::y("10"),
                        svg::attr::width("40"),
                        svg::attr::height("40"),
                        svg::attr::fill("green"),
                        svg::attr::stroke("black"),
                        svg::attr::stroke_width("2"),
                    ),
                    (),
                ),
                svg::line(
                    (
                        svg::attr::x1("20"),
                        svg::attr::y1("200"),
                        svg::attr::x2("200"),
                        svg::attr::y2("20"),
                        svg::attr::stroke("blue"),
                        svg::attr::stroke_width("10"),
                        svg::attr::stroke_linecap("round"),
                    ),
                    (),
                ),
                svg::polyline(
                    (
                        svg::attr::points(
                            "200,40 \
                             240,40 \
                             240,80 \
                             280,80 \
                             280,120 \
                             320,120 \
                             320,160",
                        ),
                        svg::attr::fill("none"),
                        svg::attr::stroke("red"),
                        svg::attr::stroke_width("4"),
                        svg::attr::stroke_dasharray("20,2"),
                    ),
                    (),
                ),
                svg::text(
                    (
                        svg::attr::x("130"),
                        svg::attr::y("130"),
                        svg::attr::fill("black"),
                        svg::attr::text_anchor("middle"),
                        svg::attr::dominant_baseline("central"),
                        svg::attr::transform("rotate(-45 130,130)"),
                    ),
                    "Welcome to Shape Club",
                ),
            ),
        )
    })?;

    Ok(())
}
