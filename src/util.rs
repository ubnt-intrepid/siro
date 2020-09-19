use crate::global::Global;
use wasm_bindgen::prelude::*;

type BuildFxHasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;

/// A type alias of associate map used within this library.
pub(crate) type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;

/// A type alias of associate set used within this library.
pub(crate) type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;

pub fn select(selector: &str) -> Option<web::Element> {
    Global::with(|g| g.document.query_selector(selector).ok())?
}

pub fn remove_children(element: &web::Element) -> Result<(), JsValue> {
    while let Some(child) = element.first_element_child() {
        element.remove_child(&*child)?;
    }
    Ok(())
}
