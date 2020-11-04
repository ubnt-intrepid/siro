use super::Subscription;
use crate::env::Env;
use futures::prelude::*;
use futures::{
    channel::mpsc,
    stream::{FusedStream, Stream},
    task::{self, Poll},
};
use gloo_events::EventListener;
use serde::Deserialize;
use std::{borrow::Cow, marker::PhantomData, pin::Pin};
use wasm_bindgen::prelude::*;

pub fn window_event<T>(event_type: impl Into<Cow<'static, str>>) -> WindowEvent<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    WindowEvent {
        event_type: event_type.into(),
        _marker: PhantomData,
    }
}

pub struct WindowEvent<T> {
    event_type: Cow<'static, str>,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Subscription for WindowEvent<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    type Msg = T;
    type Stream = WindowEventStream<T>;

    fn subscribe(self, env: &Env) -> Result<Self::Stream, JsValue> {
        let (tx, rx) = mpsc::unbounded();

        let listener = EventListener::new(&env.window, self.event_type, move |event| {
            let event: &JsValue = event.as_ref();
            let de = serde_wasm_bindgen::Deserializer::from(event.clone());
            if let Ok(msg) = T::deserialize(de) {
                tx.unbounded_send(msg).unwrap_throw();
            }
        });

        Ok(WindowEventStream {
            rx,
            _listener: Some(listener),
        })
    }
}

pub struct WindowEventStream<T> {
    rx: mpsc::UnboundedReceiver<T>,
    _listener: Option<EventListener>,
}

impl<T> Stream for WindowEventStream<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().rx.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rx.size_hint()
    }
}

impl<T> FusedStream for WindowEventStream<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
    }
}
