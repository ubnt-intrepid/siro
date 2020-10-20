use siro::subscription::{Mailbox as _, Subscribe, Subscriber, Subscription};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn interval(timeout: i32) -> impl Subscription<Msg = (), Error = JsValue> {
    SubscribeInterval { timeout }
}

struct SubscribeInterval {
    timeout: i32,
}

impl Subscription for SubscribeInterval {
    type Msg = ();
    type Error = JsValue;
    type Subscribe = IntervalSubscription;

    fn subscribe<Ctx>(self, ctx: Ctx) -> Result<Self::Subscribe, Self::Error>
    where
        Ctx: Subscriber<Msg = Self::Msg>,
    {
        let Self { timeout } = self;

        let mailbox = ctx.mailbox();

        let window = web::window().ok_or("no global `Window` exists")?;

        let cb = Closure::wrap(Box::new(move || {
            mailbox.send_message(());
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

impl Subscribe for IntervalSubscription {
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
