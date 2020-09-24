use super::Subscription;
use crate::mailbox::Mailbox;
use once_cell::unsync::OnceCell;
use std::{any::Any, cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn animation_frame<F, TMsg>(callback: F) -> impl Subscription<TMsg>
where
    F: FnMut(f64) -> TMsg + 'static,
{
    FramesSubscription { callback }
}

struct RequestFrames {
    id: Cell<Option<i32>>,
    stopped: Cell<bool>,
    cb: OnceCell<Closure<dyn FnMut(f64)>>,
    window: web::Window,
}

impl RequestFrames {
    fn new(window: web::Window) -> Self {
        Self {
            id: Cell::new(None),
            stopped: Cell::new(true),
            cb: OnceCell::new(),
            window,
        }
    }

    fn set_callback(self: &Rc<Self>, mut callback: impl FnMut(f64) + 'static) {
        let me_ref = Rc::downgrade(self);

        let f = move |timestamp| {
            callback(timestamp);

            if let Some(me) = me_ref.upgrade() {
                if !me.stopped.get() {
                    me.request_frame().unwrap_throw();
                }
            }
        };

        self.cb
            .set(Closure::wrap(Box::new(f)))
            .expect_throw("cb has already been set");
    }

    fn request_frame(&self) -> Result<(), JsValue> {
        let cb = self.cb.get().expect_throw("cb is not set");

        let id = self
            .window
            .request_animation_frame(cb.as_ref().unchecked_ref())?;

        self.id.replace(Some(id));

        Ok(())
    }

    fn start(&self) -> Result<(), JsValue> {
        self.stopped.replace(false);
        self.request_frame()?;
        Ok(())
    }

    fn stop(&self) -> Result<(), JsValue> {
        self.stopped.replace(true);

        if let Some(id) = self.id.take() {
            self.window.cancel_animation_frame(id)?;
        }

        Ok(())
    }
}

struct StopFramesOnDrop {
    frames: Rc<RequestFrames>,
}

impl Drop for StopFramesOnDrop {
    fn drop(&mut self) {
        let _ = self.frames.stop();
    }
}

struct FramesSubscription<F> {
    callback: F,
}

impl<F, TMsg> Subscription<TMsg> for FramesSubscription<F>
where
    F: FnMut(f64) -> TMsg + 'static,
{
    fn subscribe(
        self,
        window: &web::Window,
        mailbox: impl Mailbox<TMsg> + 'static,
    ) -> Result<Box<dyn Any>, JsValue> {
        let Self { mut callback } = self;

        let frames = Rc::new(RequestFrames::new(window.clone()));

        frames.set_callback(move |timestamp| {
            mailbox.send_message(callback(timestamp));
        });

        frames.start().unwrap_throw();

        Ok(Box::new(StopFramesOnDrop { frames }))
    }
}
