mod app;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use web_sys::Storage;
use wee_alloc::WeeAlloc;

use app::{Command, Model};

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const STORAGE_KEY: &str = "siro-todomvc-save";

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::new()?;
    app.mount_to_body()?;

    let storage = app
        .window()
        .local_storage()?
        .ok_or("cannot access localStorage")?;

    let mut model = restore_model(&storage).unwrap_or_default();
    model.visibility = current_visibility(app.window());

    app.render(app::view(&model))?;

    while let Some(msg) = app.next_message().await {
        let cmd = app::update(&mut model, msg);
        app.render(app::view(&model))?;

        match cmd {
            Command::FocusElement(id) => {
                let _ = focus_element(id);
            }
            Command::SaveModel => {
                let _ = save_model(&model, &storage);
            }
            Command::Nop => (),
        }
    }

    Ok(())
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

fn focus_element(id: String) -> Option<()> {
    web_sys::window()?
        .document()?
        .get_element_by_id(&id)?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?
        .focus()
        .ok()
}
