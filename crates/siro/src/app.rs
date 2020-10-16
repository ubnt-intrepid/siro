use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{self, Node, NodeCache},
};
use futures::{channel::mpsc, prelude::*};
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    document: web::Document,
    vnode: Option<Box<dyn NodeCache>>,
    mailbox: AppMailbox<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn mount(mountpoint: web::Node) -> Result<Self, JsValue> {
        let (tx, rx) = mpsc::unbounded();
        Ok(App {
            mountpoint,
            document: crate::util::document().ok_or("no Document exists")?,
            vnode: None,
            mailbox: AppMailbox { tx },
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

    pub fn render<TNode>(&mut self, node: TNode) -> Result<(), JsValue>
    where
        TNode: Node<Msg = TMsg>,
    {
        let mut ctx = RootContext {
            document: &self.document,
            mailbox: &self.mailbox,
        };

        if let Some(old) = &mut self.vnode {
            crate::vdom::diff(node, &mut ctx, old)?;
        } else {
            let vnode = node.render(&mut ctx)?;
            self.mountpoint.append_child(vnode.as_ref())?;
            self.vnode.replace(Box::new(vnode));
        }
        Ok(())
    }
}

struct RootContext<'a, TMsg: 'static> {
    document: &'a web::Document,
    mailbox: &'a AppMailbox<TMsg>,
}

impl<TMsg: 'static> vdom::Context for RootContext<'_, TMsg> {
    type Msg = TMsg;

    fn create_element(
        &mut self,
        tag_name: &str,
        namespace_uri: Option<&str>,
    ) -> Result<web::Element, JsValue> {
        match &namespace_uri {
            Some(uri) => self.document.create_element_ns(Some(uri), tag_name),
            None => self.document.create_element(tag_name),
        }
    }

    fn create_text_node(&mut self, value: &str) -> Result<web::Text, JsValue> {
        Ok(self.document.create_text_node(&*value))
    }

    fn set_listener<F>(
        &mut self,
        target: &web::EventTarget,
        event_type: &'static str,
        callback: F,
    ) -> EventListener
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static,
    {
        let sender = self.mailbox.sender();
        EventListener::new(target, event_type, move |event| {
            if let Some(msg) = callback(event) {
                sender.send_message(msg);
            }
        })
    }
}

struct AppMailbox<TMsg: 'static> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Mailbox for AppMailbox<TMsg> {
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

impl<TMsg: 'static> Mailbox for App<TMsg> {
    type Msg = TMsg;
    type Sender = AppSender<TMsg>;

    fn send_message(&self, msg: Self::Msg) {
        self.mailbox.send_message(msg);
    }

    fn sender(&self) -> Self::Sender {
        self.mailbox.sender()
    }
}
