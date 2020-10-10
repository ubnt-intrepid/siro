mod renderer;

use crate::{
    mailbox::{Mailbox, Sender},
    vdom::VNode,
    view::View,
};
use futures::{channel::mpsc, prelude::*};
use wasm_bindgen::prelude::*;

use renderer::Renderer;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    vnode: VNode,
    renderer: Renderer,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn mount(mountpoint: web::Node) -> Result<Self, JsValue> {
        let mut renderer = Renderer::new()?;

        let view: VNode = "Now rendering...".into();
        let node = renderer.render(&view)?;
        mountpoint.append_child(&node)?;

        let (tx, rx) = mpsc::unbounded();

        Ok(App {
            mountpoint,
            vnode: view,
            renderer,
            tx,
            rx,
        })
    }

    pub fn mount_to_body() -> Result<Self, JsValue> {
        let body = crate::util::document()
            .ok_or("no Document exists")?
            .body()
            .ok_or("missing body in document")?
            .into();
        Self::mount(body)
    }

    pub fn mountpoint(&self) -> &web::Node {
        &self.mountpoint
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }

    pub fn render<TView>(&mut self, view: TView) -> Result<(), JsValue>
    where
        TView: View<Msg = TMsg>,
    {
        let vnode = view.render(&*self);

        self.renderer.diff(&self.vnode, &vnode)?;
        self.vnode = vnode;

        Ok(())
    }
}

impl<TMsg: 'static> Mailbox for App<TMsg> {
    type Msg = TMsg;
    type Sender = AppSender<TMsg>;

    fn send_message(&self, msg: TMsg) {
        self.tx.unbounded_send(msg).unwrap_throw();
    }

    fn sender(&self) -> Self::Sender {
        AppSender(self.tx.clone())
    }
}

pub struct AppSender<TMsg>(mpsc::UnboundedSender<TMsg>);

impl<TMsg> Clone for AppSender<TMsg> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<TMsg: 'static> Sender for AppSender<TMsg> {
    type Msg = TMsg;

    fn send_message(&self, msg: TMsg) {
        self.0.unbounded_send(msg).unwrap_throw();
    }
}
