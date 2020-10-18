use siro::prelude::*;
use siro::App;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

mod counter {
    use siro::prelude::*;

    #[derive(Default, Clone)]
    pub struct Model {
        value: i32,
    }

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

    pub fn view(model: &Model) -> impl Node<Msg = Msg> {
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

type Model = Vec<counter::Model>;

struct Msg(usize, counter::Msg);

fn update(model: &mut Model, msg: Msg) {
    let Msg(i, msg) = msg;
    counter::update(&mut model[i], msg);
}

fn view(model: &Model) -> impl Node<Msg = Msg> + '_ {
    use siro::children::iter;
    use siro::html::div;

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

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mountpoint = siro::util::select("#app").ok_or("missing #app")?;
    let mut app = App::mount(mountpoint)?;

    let mut model = vec![counter::Model::default(); 10];
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        update(&mut model, msg);
        app.render(view(&model))?;
    }

    Ok(())
}
