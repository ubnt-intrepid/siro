use crate::{
    env::Env,
    render::{RenderContext, VNode},
};
use futures::{
    channel::mpsc, //
    prelude::*,
    stream::{FusedStream, Stream},
    task::{self, Poll},
};
use siro::vdom::Nodes;
use std::pin::Pin;

pub struct App<'env, TMsg: 'static> {
    env: &'env Env,
    mountpoint: web::Node,
    vnodes: Vec<VNode>,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<'env, TMsg: 'static> App<'env, TMsg> {
    pub(crate) fn new(env: &'env Env, mountpoint: web::Node) -> Self {
        let (tx, rx) = mpsc::unbounded();
        Self {
            env,
            mountpoint,
            vnodes: vec![],
            tx,
            rx,
        }
    }

    pub fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.next().await
    }

    pub fn render<N>(&mut self, nodes: N) -> crate::Result<()>
    where
        N: Nodes<TMsg>,
    {
        RenderContext {
            document: &self.env.document,
            parent: &self.mountpoint,
            tx: &self.tx,
        }
        .diff_nodes(nodes, &mut self.vnodes)?;
        Ok(())
    }
}

impl<TMsg: 'static> Stream for App<'_, TMsg> {
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

impl<TMsg: 'static> FusedStream for App<'_, TMsg> {
    #[inline]
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
    }
}
