use wasm_bindgen::prelude::*;
use web_sys as web;

pub fn remove_children(element: &web::Element) -> Result<(), JsValue> {
    while let Some(child) = element.first_element_child() {
        element.remove_child(&*child)?;
    }
    Ok(())
}
