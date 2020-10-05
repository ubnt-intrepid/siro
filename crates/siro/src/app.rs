use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{Renderer, VNode},
    view::View,
};
use futures::{channel::mpsc, prelude::*};
use wasm_bindgen::prelude::*;

pub trait Mountpoint {
    fn get_node(&self, document: &web::Document) -> Result<web::Node, JsValue>;
}

impl Mountpoint for str {
    fn get_node(&self, document: &web::Document) -> Result<web::Node, JsValue> {
        document
            .query_selector(self)?
            .map(Into::into)
            .ok_or(format!("cannot find mountpoint: {}", self).into())
    }
}

impl Mountpoint for web::Node {
    fn get_node(&self, _: &web::Document) -> Result<web::Node, JsValue> {
        Ok(self.clone())
    }
}

impl Mountpoint for web::Element {
    fn get_node(&self, _: &web::Document) -> Result<web::Node, JsValue> {
        Ok(self.clone().into())
    }
}

pub struct App<TMsg: 'static> {
    document: web::Document,
    mountpoint: web::Node,
    vnode: VNode,
    renderer: Renderer,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn mount(mountpoint: &(impl Mountpoint + ?Sized)) -> Result<Self, JsValue> {
        let document = crate::util::document().ok_or("no Document exists in Window")?;
        let mountpoint = mountpoint.get_node(&document)?;

        let mut renderer = Renderer::default();

        let view: VNode = "Now rendering...".into();
        let node = renderer.render(&view, &document)?;
        mountpoint.append_child(&node)?;

        let (tx, rx) = mpsc::unbounded();

        Ok(App {
            document,
            mountpoint,
            vnode: view,
            renderer,
            tx,
            rx,
        })
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

        self.renderer.diff(&self.vnode, &vnode, &self.document)?;
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
