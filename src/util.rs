use crate::global::Global;
use wasm_bindgen::prelude::*;
use web_sys as web;

pub fn select(selector: &str) -> Option<web::Element> {
    Global::with(|g| g.document.query_selector(selector).ok())?
}

pub fn remove_children(element: &web::Element) -> Result<(), JsValue> {
    while let Some(child) = element.first_element_child() {
        element.remove_child(&*child)?;
    }
    Ok(())
}
