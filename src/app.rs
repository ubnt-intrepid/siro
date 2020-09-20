use crate::{
    global::Global,
    vdom::{Node, Renderer},
};
use wasm_bindgen::prelude::*;

pub struct App {
    view: Node,
    renderer: Renderer,
}

impl App {
    pub fn mount(mountpoint: &web::Node) -> Result<Self, JsValue> {
        Global::with(|g| {
            let view: Node = "Now rendering...".into();

            let mut renderer = Renderer::default();

            let node = renderer.render(&view, &g.document)?;
            mountpoint.append_child(&node)?;

            Ok(App { view, renderer })
        })
    }

    pub fn render(&mut self, view: impl Into<Node>) -> Result<(), JsValue> {
        let view = view.into();
        Global::with(|g| self.renderer.diff(&self.view, &view, &g.document))?;
        self.view = view;
        Ok(())
    }
}
