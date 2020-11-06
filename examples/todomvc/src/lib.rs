mod app;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

use app::Model;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const STORAGE_KEY: &str = "siro-todomvc-save";

#[wasm_bindgen(start)]
pub async fn main() -> siro_web::Result<()> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    let mut model = restore_model(&env).unwrap_or_default();
    model.visibility = env
        .current_url_hash()
        .and_then(|hash| hash.trim_start_matches("#/").parse().ok());

    let mut app = env.mount_to_body()?;
    app.render(app::view(&model))?;

    while let Some(msg) = app.next_message().await {
        let focusing_elements = app::update(
            &mut model,
            msg,
            Effects {
                env: &env,
                focusing_elements: HashSet::new(),
            },
        )?;
        app.render(app::view(&model))?;

        for id in &focusing_elements {
            app.focus(id)?;
        }
    }

    Ok(())
}

struct Effects<'env> {
    env: &'env siro_web::Env,
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
        self.env.set_storage_item(STORAGE_KEY, encoded)?;
        Ok(())
    }
}

fn restore_model(env: &siro_web::Env) -> Option<app::Model> {
    let model_raw = env.get_storage_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&model_raw).ok()
}
