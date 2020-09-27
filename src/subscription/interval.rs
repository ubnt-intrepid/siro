use super::Subscription;
use crate::mailbox::{Mailbox, Sender as _};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn interval(timeout: i32) -> Interval {
    Interval { timeout }
}

pub struct Interval {
    timeout: i32,
}

impl Subscription for Interval {
    type Msg = ();
    type Handle = Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
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

        Ok(Handle {
            window: window.clone(),
            id,
            _cb: cb,
        })
    }
}

pub struct Handle {
    window: web::Window,
    id: i32,
    _cb: Closure<dyn FnMut()>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.window.clear_interval_with_handle(self.id);
    }
}
