use gloo_events::EventListener;
use siro::{
    event::Event,
    subscription::{Mailbox as _, Subscribe, Subscriber, Subscription},
};
use std::borrow::Cow;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct WindowEvent(web::Event);

impl AsRef<web::Event> for WindowEvent {
    #[inline]
    fn as_ref(&self) -> &web::Event {
        &self.0
    }
}

impl std::ops::Deref for WindowEvent {
    type Target = web::Event;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'e> Event<'e> for WindowEvent {
    type Deserializer = serde_wasm_bindgen::Deserializer;
    type Error = serde_wasm_bindgen::Error;

    fn into_deserializer(self) -> Self::Deserializer {
        let value: JsValue = self.0.into();
        serde_wasm_bindgen::Deserializer::from(value)
    }
}

pub fn window_event(
    event_type: impl Into<Cow<'static, str>>,
) -> impl Subscription<Msg = WindowEvent, Error = JsValue> {
    SubscribeWindowEvent {
        event_type: event_type.into(),
    }
}

struct SubscribeWindowEvent {
    event_type: Cow<'static, str>,
}

impl Subscription for SubscribeWindowEvent {
    type Msg = WindowEvent;
    type Subscribe = WindowEventSubscription;
    type Error = JsValue;

    fn subscribe<Ctx>(self, ctx: Ctx) -> Result<Self::Subscribe, Self::Error>
    where
        Ctx: Subscriber<Msg = Self::Msg>,
    {
        let Self { event_type } = self;

        let mailbox = ctx.mailbox();

        let window = web::window().ok_or("no global `Window` exists")?;

        let listener = EventListener::new(&window, event_type, move |event| {
            mailbox.send_message(WindowEvent(event.clone()));
        });

        Ok(WindowEventSubscription {
            listener: Some(listener),
        })
    }
}

struct WindowEventSubscription {
    listener: Option<EventListener>,
}

impl Subscribe for WindowEventSubscription {
    type Error = JsValue;

    fn unsubscribe(&mut self) -> Result<(), Self::Error> {
        drop(self.listener.take());
        Ok(())
    }
}
