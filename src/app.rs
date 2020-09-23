use crate::{
    mailbox::Mailbox,
    vdom::{Node, Renderer},
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
    window: web::Window,
    document: web::Document,
    mountpoint: web::Node,
    view: Node,
    renderer: Renderer,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn mount(mountpoint: &(impl Mountpoint + ?Sized)) -> Result<Self, JsValue> {
        let window = web::window().ok_or("no global Window exists")?;
        let document = window.document().ok_or("no Document exists in Window")?;
        let mountpoint = mountpoint.get_node(&document)?;

        let mut renderer = Renderer::default();

        let view: Node = "Now rendering...".into();
        let node = renderer.render(&view, &document)?;
        mountpoint.append_child(&node)?;

        let (tx, rx) = mpsc::unbounded();

        Ok(App {
            window,
            document,
            mountpoint,
            view,
            renderer,
            tx,
            rx,
        })
    }

    pub fn mountpoint(&self) -> &web::Node {
        &self.mountpoint
    }

    pub fn mailbox(&self) -> impl Mailbox<TMsg> {
        self.tx.clone()
    }

    pub fn window(&self) -> &web::Window {
        &self.window
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }

    pub fn render(&mut self, view: impl Into<Node>) -> Result<(), JsValue> {
        let view = view.into();
        self.renderer.diff(&self.view, &view, &self.document)?;
        self.view = view;
        Ok(())
    }
}
