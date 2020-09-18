use crate::{
    app::App,
    vdom::{self, CachedNodes},
};
use wasm_bindgen::prelude::*;
use web_sys as web;

// ==== Meow ====

/// Initialize a new instance of global context.
pub fn global_context() -> Option<GlobalContext> {
    let window = web::window()?;
    let document = window.document()?;
    Some(GlobalContext {
        _window: window,
        document,
    })
}

pub struct GlobalContext {
    _window: web::Window,
    document: web::Document,
}

impl GlobalContext {
    pub fn select(&self, selector: &str) -> Option<web::Element> {
        self.document.query_selector(selector).ok()?
    }

    pub(crate) fn document(&self) -> &web::Document {
        &self.document
    }

    pub fn mount(&self, mountpoint: &web::Node) -> Result<App, JsValue> {
        let view = vdom::text("Now rendering...").into();

        let mut caches = CachedNodes::default();
        let mut event_listeners = vec![];

        let node = vdom::render(&view, &self.document, &mut caches, &mut event_listeners)?;
        mountpoint.append_child(&node)?;

        Ok(App {
            view,
            caches,
            event_listeners,
        })
    }
}
