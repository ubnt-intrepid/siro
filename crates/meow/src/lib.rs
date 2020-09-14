#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

pub mod vdom;

use wasm_bindgen::prelude::*;
use web_sys as web;

// ==== View ====

pub trait View<Model>: 'static {
    fn render(&self, model: &Model) -> vdom::Node;
}

impl<F, Model, R> View<Model> for F
where
    F: Fn(&Model) -> R + 'static,
    R: Into<vdom::Node>,
{
    fn render(&self, model: &Model) -> vdom::Node {
        (*self)(model).into()
    }
}

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

    pub fn mount<TModel, TView>(
        &self,
        mountpoint: impl Mountpoint,
        model: TModel,
        view: TView,
    ) -> Result<App<TModel, TView>, JsValue>
    where
        TView: View<TModel>,
    {
        let mountpoint = mountpoint
            .get_node(self)
            .ok_or("cannot get mountpoint node")?;

        while let Some(child) = mountpoint.first_child() {
            mountpoint.remove_child(&child)?;
        }

        let mut vnode = vdom::Text::new("Now rendering...");
        let node = vnode.create_node(&*self);

        mountpoint.append_child(&node)?;

        Ok(App {
            node,
            vnode: vnode.into(),
            model,
            view,
        })
    }
}

// ==== App ====

pub struct App<TModel, TView> {
    node: web::Node,
    vnode: vdom::Node,
    model: TModel,
    view: TView,
}

impl<TModel, TView> App<TModel, TView>
where
    TView: View<TModel>,
{
    pub fn render(&mut self, meow: &Meow) {
        let new_vnode = self.view.render(&self.model);
        let new_node = self.vnode.apply_patch(meow, new_vnode);
        self.node = new_node;
    }
}
