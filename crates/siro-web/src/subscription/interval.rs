use siro::{
    mailbox::{Mailbox, Sender as _},
    subscription::{Subscribe, Subscription},
};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn interval(timeout: i32) -> impl Subscribe<Msg = (), Error = JsValue> {
    SubscribeInterval { timeout }
}

struct SubscribeInterval {
    timeout: i32,
}

impl Subscribe for SubscribeInterval {
    type Msg = ();
    type Error = JsValue;
    type Subscription = IntervalSubscription;

    fn subscribe<M: ?Sized>(self, mailbox: &M) -> Result<Self::Subscription, Self::Error>
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        let Self { timeout } = self;

        let sender = mailbox.sender();

        let window = web::window().ok_or("no global `Window` exists")?;

        let cb = Closure::wrap(Box::new(move || {
            sender.send_message(());
        }) as Box<dyn FnMut()>);

        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            timeout,
        )?;

        Ok(IntervalSubscription(Some(Inner {
            window: window.clone(),
            id,
            _cb: cb,
        })))
    }
}

struct IntervalSubscription(Option<Inner>);

struct Inner {
    window: web::Window,
    id: i32,
    _cb: Closure<dyn FnMut()>,
}

impl Subscription for IntervalSubscription {
    type Msg = ();
    type Error = JsValue;

    fn unsubscribe(&mut self) -> Result<(), Self::Error> {
        if let Some(inner) = self.0.take() {
            inner.window.clear_interval_with_handle(inner.id);
        }
        Ok(())
    }
}

impl Drop for IntervalSubscription {
    fn drop(&mut self) {
        let _ = self.unsubscribe();
    }
}
