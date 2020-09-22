use crate::{
    global::Global,
    mailbox::Mailbox,
    vdom::{Node, Renderer},
};
use futures::{channel::mpsc, prelude::*};
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    view: Node,
    renderer: Renderer,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn mount(mountpoint: &web::Node) -> Result<Self, JsValue> {
        Global::with(|g| {
            let view: Node = "Now rendering...".into();

            let mut renderer = Renderer::default();

            let node = renderer.render(&view, &g.document)?;
            mountpoint.append_child(&node)?;

            let (tx, rx) = mpsc::unbounded();

            Ok(App {
                view,
                renderer,
                tx,
                rx,
            })
        })
    }

    pub fn mailbox(&self) -> impl Mailbox<TMsg> {
        self.tx.clone()
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }

    pub fn render(&mut self, view: impl Into<Node>) -> Result<(), JsValue> {
        let view = view.into();
        Global::with(|g| self.renderer.diff(&self.view, &view, &g.document))?;
        self.view = view;
        Ok(())
    }
}
