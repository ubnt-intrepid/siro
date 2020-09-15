#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

pub mod vdom;

use wasm_bindgen::prelude::*;
use web_sys as web;

// ==== Mountpoint ====

pub trait Mountpoint {
    fn get_node(&self, meow: &Meow) -> Option<web::Node>;
}

impl Mountpoint for &str {
    fn get_node(&self, meow: &Meow) -> Option<web::Node> {
        meow.document.query_selector(self).ok()?.map(Into::into)
    }
}

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

    pub fn scene(
        &self,
        mountpoint: impl Mountpoint,
        initial_view: impl Into<vdom::Node>,
    ) -> Result<Scene, JsValue> {
        let mountpoint = mountpoint
            .get_node(self)
            .ok_or("cannot get mountpoint node")?;

        let mut view = initial_view.into();
        let dom_node = view.render(&*self);

        mountpoint.append_child(&dom_node)?;

        Ok(Scene { dom_node, view })
    }
}

// ==== App ====

pub struct Scene {
    dom_node: web::Node,
    view: vdom::Node,
}

impl Scene {
    pub fn set_view(&mut self, meow: &Meow, view: impl Into<vdom::Node>) -> Result<(), JsValue> {
        let view = view.into();
        let dom_node = self.view.diff(meow, view);
        self.dom_node = dom_node;
        Ok(())
    }
}
