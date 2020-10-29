use crate::render::{RenderContext, VNode};
use futures::{
    channel::mpsc,
    prelude::*,
    stream::FusedStream,
    task::{self, Poll},
};
use siro::{
    subscription::{Mailbox, Subscriber, Subscription},
    vdom::Nodes,
};
use std::pin::Pin;
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    document: web::Document,
    vnodes: Vec<VNode>,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    fn new(mountpoint: web::Node, document: web::Document) -> Self {
        let (tx, rx) = mpsc::unbounded();
        Self {
            mountpoint,
            document,
            vnodes: vec![],
            tx,
            rx,
        }
    }

    pub fn mount(selector: &str) -> Result<Self, JsValue> {
        let document = crate::document().ok_or("no Document exists")?;
        let mountpoint = document.query_selector(selector)?.ok_or("missing node")?;
        Ok(Self::new(mountpoint.into(), document))
    }

    pub fn mount_to_body() -> Result<Self, JsValue> {
        let document = crate::document().ok_or("no Document exists")?;
        let body = document.body().ok_or("missing body in document")?;
        Ok(Self::new(body.into(), document))
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
        RenderContext {
            document: &self.document,
            parent: &self.mountpoint,
            subscriber: AppSubscriber { tx: &self.tx },
        }
        .diff_nodes(nodes, &mut self.vnodes)?;
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
