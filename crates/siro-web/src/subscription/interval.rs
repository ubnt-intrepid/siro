use super::Subscription;
use crate::env::Env;
use futures::prelude::*;
use futures::{
    channel::mpsc,
    stream::{FusedStream, Stream},
    task::{self, Poll},
};
use std::pin::Pin;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;

#[inline]
pub fn interval(timeout: i32) -> Interval {
    Interval { timeout }
}

pub struct Interval {
    timeout: i32,
}

impl Subscription for Interval {
    type Msg = ();
    type Stream = IntervalStream;

    fn subscribe(self, env: &Env) -> Result<Self::Stream, JsValue> {
        let (tx, rx) = mpsc::unbounded();

        let cb = Closure::wrap(Box::new(move || {
            tx.unbounded_send(()).unwrap_throw();
        }) as Box<dyn FnMut()>);

        let id = env
            .window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                self.timeout,
            )?;

        Ok(IntervalStream {
            rx,
            inner: Some(Inner {
                window: env.window.clone(),
                id,
                _cb: cb,
            }),
        })
    }
}

pub struct IntervalStream {
    rx: mpsc::UnboundedReceiver<()>,
    inner: Option<Inner>,
}

struct Inner {
    window: web::Window,
    id: i32,
    _cb: Closure<dyn FnMut()>,
}

impl IntervalStream {
    fn unsubscribe(&mut self) -> Result<(), JsValue> {
        if let Some(inner) = self.inner.take() {
            inner.window.clear_interval_with_handle(inner.id);
        }
        Ok(())
    }
}

impl Drop for IntervalStream {
    fn drop(&mut self) {
        let _ = self.unsubscribe();
    }
}

impl Stream for IntervalStream {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().rx.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rx.size_hint()
    }
}

impl FusedStream for IntervalStream {
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
    }
}
