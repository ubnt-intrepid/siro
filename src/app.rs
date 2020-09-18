use crate::{
    global::GlobalContext,
    vdom::{self, CachedNodes, Node},
};
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;

pub struct App {
    pub(crate) view: Node,
    pub(crate) caches: CachedNodes,
    pub(crate) event_listeners: Vec<EventListener>,
}

impl App {
    pub fn render(&mut self, meow: &GlobalContext, view: impl Into<Node>) -> Result<(), JsValue> {
        let view = view.into();

        // FIXME: more efficient
        for listener in self.event_listeners.drain(..) {
            drop(listener);
        }

        vdom::diff(
            &self.view,
            &view,
            meow.document(),
            &mut self.caches,
            &mut self.event_listeners,
        )?;

        self.view = view;

        Ok(())
    }
}
