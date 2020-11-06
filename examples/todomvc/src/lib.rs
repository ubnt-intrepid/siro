mod app;

use std::collections::HashSet;
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
        let focusing_elements = app::update(
            &mut model,
            msg,
            Effects {
                storage: &storage,
                focusing_elements: HashSet::new(),
            },
        )?;
        app.render(app::view(&model))?;

        for id in &focusing_elements {
            let _ = focus_element(id);
        }
    }

    Ok(())
}

struct Effects<'a> {
    storage: &'a Storage,
    focusing_elements: HashSet<String>,
}

impl siro::effects::Effects for Effects<'_> {
    type Ok = HashSet<String>;
    type Error = siro_web::Error;

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.focusing_elements)
    }
}

impl siro::effects::DomFocus for Effects<'_> {
    fn focus(&mut self, target_id: &str) -> Result<(), Self::Error> {
        self.focusing_elements.insert(target_id.to_owned());
        Ok(())
    }
}

impl app::SaveModel for Effects<'_> {
    fn save_model(&mut self, model: &Model) -> Result<(), Self::Error> {
        let encoded = serde_json::to_string(model).expect_throw("Model serialize");
        self.storage
            .set_item(STORAGE_KEY, &encoded)
            .map_err(siro_web::Error::caught_from_js)?;
        Ok(())
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
