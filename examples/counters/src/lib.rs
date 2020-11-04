use siro::prelude::*;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

mod counter {
    use siro::prelude::*;

    // ==== model ====

    #[derive(Default, Clone)]
    pub struct Model {
        value: i32,
    }

    // ==== update ====

    pub enum Msg {
        Increment,
        Decrement,
        Reset,
    }

    pub fn update(model: &mut Model, msg: Msg) {
        match msg {
            Msg::Increment => model.value += 1,
            Msg::Decrement => model.value -= 1,
            Msg::Reset => model.value = 0,
        }
    }

    // ==== view ====

    pub fn view(model: &Model) -> impl Nodes<Msg> {
        use siro::html::{button, div, event::on_click};

        div(
            (),
            (
                button(on_click(|| Msg::Decrement), "-"),
                " ",
                model.value.to_string(),
                " ",
                button(on_click(|| Msg::Increment), "+"),
                " ",
                button(on_click(|| Msg::Reset), "Reset"),
            ),
        )
    }
}

// ==== model ====

type Model = Vec<counter::Model>;

// ==== update ====

struct Msg(usize, counter::Msg);

fn update(model: &mut Model, msg: Msg) {
    let Msg(i, msg) = msg;
    counter::update(&mut model[i], msg);
}

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> + '_ {
    use siro::{html::div, vdom::iter};

    div(
        (),
        iter(
            model
                .iter() //
                .enumerate()
                .map(|(i, m)| {
                    div(
                        (),
                        (
                            format!("{}: ", i),
                            counter::view(m).map(move |msg| Msg(i, msg)),
                        ),
                    )
                }),
        ),
    )
}

// ==== runtime ====

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount("#app")?;

    let mut model = vec![counter::Model::default(); 10];
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
