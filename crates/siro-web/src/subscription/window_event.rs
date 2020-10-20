use gloo_events::EventListener;
use serde::Deserialize;
use siro::subscription::{Mailbox as _, Subscribe, Subscriber, Subscription};
use std::{borrow::Cow, marker::PhantomData};
use wasm_bindgen::prelude::*;

pub fn window_event<T>(
    event_type: impl Into<Cow<'static, str>>,
) -> impl Subscription<Msg = T, Error = JsValue>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    SubscribeWindowEvent {
        event_type: event_type.into(),
        _marker: PhantomData,
    }
}

struct SubscribeWindowEvent<T> {
    event_type: Cow<'static, str>,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Subscription for SubscribeWindowEvent<T>
where
    T: for<'de> Deserialize<'de> + 'static,
{
    type Msg = T;
    type Subscribe = WindowEventSubscription;
    type Error = JsValue;

    fn subscribe<Ctx>(self, ctx: Ctx) -> Result<Self::Subscribe, Self::Error>
    where
        Ctx: Subscriber<Msg = Self::Msg>,
    {
        let Self { event_type, .. } = self;

        let mailbox = ctx.mailbox();

        let window = web::window().ok_or("no global `Window` exists")?;

        let listener = EventListener::new(&window, event_type, move |event| {
            let event: &JsValue = event.as_ref();
            let de = serde_wasm_bindgen::Deserializer::from(event.clone());
            if let Ok(msg) = T::deserialize(de) {
                mailbox.send_message(msg);
            }
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
