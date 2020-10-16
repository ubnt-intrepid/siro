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

pub fn select(selector: &str) -> Option<web::Node> {
    document()?.query_selector(selector).ok()?.map(Into::into)
}
