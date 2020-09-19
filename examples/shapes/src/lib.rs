use siro::vdom::svg;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Instantiate the Siro global context.
    let ctx = siro::global_context() //
        .ok_or("cannot create global context")?;

    // Find the mountpoint node for the application in the DOM.
    let mountpoint = ctx
        .select("#app") //
        .ok_or("cannot find `#app` in document")?;

    // Remove all childlen from mountpoint node.
    siro::util::remove_children(&mountpoint)?;

    // Mount a Siro application on the specified mountpoint.
    let mut app = ctx.mount(mountpoint.as_ref())?;

    // Draw the virtual DOM.
    app.render(&ctx, {
        svg("svg") //
            .attribute("viewbox", "0 0 400 400")
            .attribute("width", "400")
            .attribute("height", "400")
            .child(
                svg("circle")
                    .attribute("cx", "50")
                    .attribute("cy", "50")
                    .attribute("r", "40")
                    .attribute("fill", "red")
                    .attribute("stroke", "black")
                    .attribute("stroke-width", "3"),
            )
            .child(
                svg("rect")
                    .attribute("x", "100")
                    .attribute("y", "10")
                    .attribute("width", "40")
                    .attribute("height", "40")
                    .attribute("fill", "green")
                    .attribute("stroke", "black")
                    .attribute("stroke-width", "2"),
            )
            .child(
                svg("line")
                    .attribute("x1", "20")
                    .attribute("y1", "200")
                    .attribute("x2", "200")
                    .attribute("y2", "20")
                    .attribute("stroke", "blue")
                    .attribute("stroke-width", "10")
                    .attribute("stroke-linecap", "round"),
            )
            .child(
                svg("polyline")
                    .attribute(
                        "points",
                        "200,40 240,40 240,80 280,80 280,120 320,120 320,160",
                    )
                    .attribute("fill", "none")
                    .attribute("stroke", "red")
                    .attribute("stroke-width", "4")
                    .attribute("stroke-dasharray", "20,2"),
            )
            .child(
                svg("text")
                    .attribute("x", "130")
                    .attribute("y", "130")
                    .attribute("fill", "black")
                    .attribute("text-anchor", "middle")
                    .attribute("dominant-baseline", "central")
                    .attribute("transform", "rotate(-45 130,130)")
                    .child("Welcome to Shape Club"),
            )
    })?;

    Ok(())
}
