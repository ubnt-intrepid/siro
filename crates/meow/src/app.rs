use crate::{
    meow::Meow,
    vdom::{self, CachedNodes, Node},
};
use wasm_bindgen::prelude::*;

pub struct App {
    pub(crate) view: Node,
    pub(crate) caches: CachedNodes,
}

impl App {
    pub fn draw(&mut self, meow: &Meow, view: impl Into<Node>) -> Result<(), JsValue> {
        let view = view.into();
        vdom::diff(&self.view, &view, meow.document(), &mut self.caches)?;
        self.view = view;
        Ok(())
    }
}
