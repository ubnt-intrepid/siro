use crate::{
    render::{RenderContext, VNode},
    subscription::{Mailbox, Subscriber, Subscription},
};
use futures::{
    channel::mpsc, //
    prelude::*,
    stream::{FusedStream, Stream},
    task::{self, Poll},
};
use siro::vdom::Nodes;
use std::pin::Pin;
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    window: web::Window,
    document: web::Document,
    mountpoint: Option<web::Node>,
    vnodes: Vec<VNode>,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn new() -> Result<Self, JsValue> {
        let window = web::window().ok_or("no global Window exists")?;
        let document = window.document().ok_or("no Document exists")?;
        let (tx, rx) = mpsc::unbounded();
        Ok(Self {
            window,
            document,
            mountpoint: None,
            vnodes: vec![],
            tx,
            rx,
        })
    }

    pub fn window(&self) -> &web::Window {
        &self.window
    }

    pub fn mount(&mut self, selector: &str) -> Result<(), JsValue> {
        let mountpoint = self
            .document
            .query_selector(selector)?
            .ok_or("missing node")?;
        self.mountpoint.replace(mountpoint.into());
        Ok(())
    }

    pub fn mount_to_body(&mut self) -> Result<(), JsValue> {
        let body = self.document.body().ok_or("missing body in document")?;
        self.mountpoint.replace(body.into());
        Ok(())
    }

    pub fn current_url_hash(&self) -> Option<String> {
        self.window.location().hash().ok()
    }

    /// Register a `Subscription`.
    pub fn subscribe<S>(&self, subscription: S) -> Result<S::Subscribe, S::Error>
    where
        S: Subscription<Msg = TMsg>,
    {
        subscription.subscribe(AppSubscriber { tx: &self.tx })
    }

    pub fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.next().await
    }

    pub fn render<N>(&mut self, nodes: N) -> Result<(), JsValue>
    where
        N: Nodes<TMsg>,
    {
        if let Some(mountpoint) = &self.mountpoint {
            RenderContext {
                document: &self.document,
                parent: mountpoint,
                subscriber: AppSubscriber { tx: &self.tx },
            }
            .diff_nodes(nodes, &mut self.vnodes)?;
        }
        Ok(())
    }
}

impl<TMsg: 'static> Stream for App<TMsg> {
    type Item = TMsg;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rx.size_hint()
    }
}

impl<TMsg: 'static> FusedStream for App<TMsg> {
    #[inline]
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
    }
}

struct AppSubscriber<'a, TMsg: 'static> {
    tx: &'a mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Subscriber for AppSubscriber<'_, TMsg> {
    type Msg = TMsg;
    type Mailbox = AppMailbox<TMsg>;

    #[inline]
    fn mailbox(&self) -> Self::Mailbox {
        AppMailbox {
            tx: self.tx.clone(),
        }
    }
}

struct AppMailbox<TMsg> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Mailbox for AppMailbox<TMsg> {
    type Msg = TMsg;

    fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }
}
