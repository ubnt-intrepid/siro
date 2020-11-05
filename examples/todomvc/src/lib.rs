mod app;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use web_sys::Storage;
use wee_alloc::WeeAlloc;

use app::Model;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const STORAGE_KEY: &str = "siro-todomvc-save";

#[wasm_bindgen(start)]
pub async fn main() -> siro_web::Result<()> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut app = env.mount_to_body()?;

    let storage = env
        .window()
        .local_storage()
        .map_err(siro_web::Error::caught_from_js)?
        .ok_or_else(|| siro_web::Error::custom("cannot access localStorage"))?;

    let mut model = restore_model(&storage).unwrap_or_default();
    model.visibility = current_visibility(env.window());

    app.render(app::view(&model))?;

    while let Some(msg) = app.next_message().await {
        let cmd = app::update(&mut model, msg, CmdBuilder::default()).expect("infallible");
        app.render(app::view(&model))?;

        if cmd.need_save_model {
            let _ = save_model(&model, &storage);
        }
        for id in &cmd.focusing_elements {
            let _ = focus_element(id);
        }
    }

    Ok(())
}

#[derive(Default)]
struct CmdBuilder {
    need_save_model: bool,
    focusing_elements: std::collections::HashSet<String>,
}

impl app::Effects for CmdBuilder {
    type Ok = Self;
    type Error = std::convert::Infallible;

    fn save_model(&mut self) -> Result<(), Self::Error> {
        self.need_save_model = true;
        Ok(())
    }

    fn focus_element(&mut self, id: &str) -> Result<(), Self::Error> {
        self.focusing_elements.insert(id.to_owned());
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self)
    }
}

fn save_model(model: &Model, storage: &Storage) {
    // ignore errors
    if let Ok(encoded) = serde_json::to_string(model) {
        let _ = storage.set_item(STORAGE_KEY, &encoded);
    }
}

fn restore_model(storage: &Storage) -> Option<app::Model> {
    let model_raw = storage.get_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&model_raw).ok()
}

fn current_visibility(window: &web_sys::Window) -> Option<app::Visibility> {
    let hash = window.location().hash().ok()?;
    hash.trim_start_matches("#/").parse().ok()
}

fn focus_element(id: &str) -> Option<()> {
    web_sys::window()?
        .document()?
        .get_element_by_id(id)?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?
        .focus()
        .ok()
}
