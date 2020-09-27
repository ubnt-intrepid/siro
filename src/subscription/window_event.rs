use super::Subscription;
use crate::mailbox::{Mailbox, Sender as _};
use gloo_events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

pub fn window_event(event_type: impl Into<Cow<'static, str>>) -> WindowEvent {
    WindowEvent {
        event_type: event_type.into(),
    }
}

pub struct WindowEvent {
    event_type: Cow<'static, str>,
}

impl Subscription for WindowEvent {
    type Msg = web::Event;
    type Handle = Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        let Self { event_type } = self;

        let sender = mailbox.sender();

        let window = web::window().ok_or("no global `Window` exists")?;

        let listener = EventListener::new(&window, event_type, move |event| {
            sender.send_message(event.clone());
        });

        Ok(Handle {
            _listener: listener,
        })
    }
}

pub struct Handle {
    _listener: EventListener,
}
