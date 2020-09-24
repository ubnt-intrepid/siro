use wasm_bindgen::prelude::*;

pub fn document() -> Option<web::Document> {
    web::window()?.document()
}

pub fn remove_children(element: &web::Element) -> Result<(), JsValue> {
    while let Some(child) = element.first_element_child() {
        element.remove_child(&*child)?;
    }
    Ok(())
}

pub struct LogOnDrop(pub &'static str);

impl Drop for LogOnDrop {
    fn drop(&mut self) {
        web::console::log_1(&self.0.into());
    }
}
