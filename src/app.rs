use crate::{
    global::Global,
    vdom::{self, CachedNodes, Node},
};
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;

pub struct App {
    view: Node,
    caches: CachedNodes,
    event_listeners: Vec<EventListener>,
}

impl App {
    pub fn mount(mountpoint: &web::Node) -> Result<Self, JsValue> {
        Global::with(|g| {
            let view = vdom::text("Now rendering...").into();

            let mut caches = CachedNodes::default();
            let mut event_listeners = vec![];

            let node = vdom::render(&view, &g.document, &mut caches, &mut event_listeners)?;
            mountpoint.append_child(&node)?;

            Ok(App {
                view,
                caches,
                event_listeners,
            })
        })
    }

    pub fn render(&mut self, view: impl Into<Node>) -> Result<(), JsValue> {
        let view = view.into();

        // FIXME: more efficient
        for listener in self.event_listeners.drain(..) {
            drop(listener);
        }

        Global::with(|g| {
            vdom::diff(
                &self.view,
                &view,
                &g.document,
                &mut self.caches,
                &mut self.event_listeners,
            )
        })?;

        self.view = view;

        Ok(())
    }
}
