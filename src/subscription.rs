use crate::mailbox::Mailbox;
use std::any::Any;
use wasm_bindgen::{prelude::*, JsCast as _};

pub trait Subscription<TMsg> {
    fn subscribe(
        self,
        window: &web::Window,
        mailbox: impl Mailbox<TMsg> + 'static,
    ) -> Result<Box<dyn Any>, JsValue>;
}

pub fn interval<F, TMsg>(timeout: i32, callback: F) -> impl Subscription<TMsg>
where
    F: FnMut() -> TMsg + 'static,
{
    struct Interval<F> {
        timeout: i32,
        callback: F,
    }

    impl<F, TMsg> Subscription<TMsg> for Interval<F>
    where
        F: FnMut() -> TMsg + 'static,
    {
        fn subscribe(
            self,
            window: &web::Window,
            mailbox: impl Mailbox<TMsg> + 'static,
        ) -> Result<Box<dyn Any>, JsValue> {
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

            struct Guard {
                window: web::Window,
                id: i32,
                _cb: Closure<dyn FnMut()>,
            }

            impl Drop for Guard {
                fn drop(&mut self) {
                    self.window.clear_interval_with_handle(self.id);
                }
            }

            Ok(Box::new(Guard {
                window: window.clone(),
                id,
                _cb: cb,
            }))
        }
    }

    Interval { timeout, callback }
}
