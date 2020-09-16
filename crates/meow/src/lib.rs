#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

pub mod vdom;

use wasm_bindgen::prelude::*;
use web_sys as web;

// ==== Meow ====

pub struct Meow {
    _window: web::Window,
    document: web::Document,
}

impl Meow {
    /// Create a new instance of `Meow`.
    pub fn init() -> Result<Self, JsValue> {
        let window = web::window().ok_or("no global `window` exists")?;

        let document = window
            .document()
            .ok_or("should have a document on window")?;

        Ok(Self {
            _window: window,
            document,
        })
    }

    pub fn select(&self, selector: &str) -> Option<web::Node> {
        self.document.query_selector(selector).ok()?.map(Into::into)
    }

    pub fn scene(
        &self,
        mountpoint: &web::Node,
        initial_view: impl Into<vdom::Node>,
    ) -> Result<Scene, JsValue> {
        let mut caches = vdom::NodeCaches::default();
        let view = initial_view.into();
        let node = view.render(&*self, &mut caches);
        mountpoint.append_child(&node)?;

        Ok(Scene { view, caches })
    }
}

// ==== App ====

pub struct Scene {
    view: vdom::Node,
    caches: vdom::NodeCaches,
}

impl Scene {
    pub fn set_view(&mut self, meow: &Meow, view: impl Into<vdom::Node>) -> Result<(), JsValue> {
        let view = view.into();
        self.view.diff(&view, meow, &mut self.caches);
        self.view = view;
        Ok(())
    }
}
