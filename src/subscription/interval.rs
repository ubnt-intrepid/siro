use super::Subscription;
use crate::mailbox::Sender;
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn interval<F, TMsg>(timeout: i32, callback: F) -> impl Subscription<TMsg>
where
    F: FnMut() -> TMsg + 'static,
{
    Interval { timeout, callback }
}

struct Interval<F> {
    timeout: i32,
    callback: F,
}

impl<F, TMsg> Subscription<TMsg> for Interval<F>
where
    F: FnMut() -> TMsg + 'static,
{
    type Handle = Handle;

    fn subscribe(
        self,
        window: &web::Window,
        mailbox: impl Sender<TMsg> + 'static,
    ) -> Result<Self::Handle, JsValue> {
        let Self {
            timeout,
            mut callback,
        } = self;

        let cb = Closure::wrap(Box::new(move || {
            mailbox.send_message(callback());
        }) as Box<dyn FnMut()>);

        let id = window.set_interval_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            timeout,
        )?;

        Ok(Handle {
            window: window.clone(),
            id,
            _cb: cb,
        })
    }
}

struct Handle {
    window: web::Window,
    id: i32,
    _cb: Closure<dyn FnMut()>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.window.clear_interval_with_handle(self.id);
    }
}
