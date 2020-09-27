use super::Subscription;
use crate::mailbox::{Mailbox, Sender as _};
use gloo_events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

pub fn window_event<F, TMsg>(
    event_type: impl Into<Cow<'static, str>>,
    callback: F,
) -> impl Subscription<TMsg>
where
    F: Fn(&web::Event) -> Option<TMsg> + 'static,
{
    WindowEvent {
        event_type: event_type.into(),
        callback,
    }
}

struct WindowEvent<F> {
    event_type: Cow<'static, str>,
    callback: F,
}

impl<F, TMsg> Subscription<TMsg> for WindowEvent<F>
where
    F: Fn(&web::Event) -> Option<TMsg> + 'static,
{
    type Handle = Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<TMsg>,
    {
        let Self {
            event_type,
            callback,
        } = self;

        let sender = mailbox.sender();

        let window = web::window().ok_or("no global `Window` exists")?;

        let listener = EventListener::new(&window, event_type, move |event| {
            if let Some(msg) = callback(&event) {
                sender.send_message(msg);
            }
        });

        Ok(Handle {
            _listener: listener,
        })
    }
}

struct Handle {
    _listener: EventListener,
}
