use either::Either;
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

pub fn if_then<T>(pred: bool, f: impl FnOnce() -> T) -> Option<T> {
    if pred {
        Some(f())
    } else {
        None
    }
}

pub fn if_else<T, U>(pred: bool, f: impl FnOnce() -> T, g: impl FnOnce() -> U) -> Either<T, U> {
    if pred {
        Either::Left(f())
    } else {
        Either::Right(g())
    }
}
