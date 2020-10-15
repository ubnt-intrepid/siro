use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{CowStr, VElement, VNode, VText},
    view::{self, View},
};
use futures::{channel::mpsc, prelude::*};
use gloo_events::EventListener;
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    document: web::Document,
    vnode: Option<VNode>,
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

    pub fn render<TView>(&mut self, view: TView) -> Result<(), JsValue>
    where
        TView: View<Msg = TMsg>,
    {
        let mut ctx = RootContext {
            document: &self.document,
            mailbox: &self.mailbox,
        };

        if let Some(old) = &mut self.vnode {
            view.diff(&mut ctx, old)?;
        } else {
            let vnode = view.render(&mut ctx)?;
            self.mountpoint.append_child(vnode.as_node())?;
            self.vnode.replace(vnode);
        }
        Ok(())
    }
}

struct RootContext<'a, TMsg: 'static> {
    document: &'a web::Document,
    mailbox: &'a AppMailbox<TMsg>,
}

impl<TMsg: 'static> view::Context for RootContext<'_, TMsg> {
    type Msg = TMsg;

    fn create_element(
        &mut self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<VElement, JsValue> {
        let node = match &namespace_uri {
            Some(uri) => self.document.create_element_ns(Some(&*uri), &*tag_name)?,
            None => self.document.create_element(&*tag_name)?,
        };
        Ok(VElement::new(node, tag_name, namespace_uri))
    }

    fn create_text_node(&mut self, value: CowStr) -> Result<VText, JsValue> {
        let node = self.document.create_text_node(&*value);
        Ok(VText { value, node })
    }

    fn create_listener<F>(
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
