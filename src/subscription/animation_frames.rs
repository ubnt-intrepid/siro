use super::Subscription;
use crate::mailbox::{Mailbox, Sender as _};
use once_cell::unsync::OnceCell;
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn animation_frames() -> AnimationFrames {
    AnimationFrames { _p: () }
}

pub struct AnimationFrames {
    _p: (),
}

impl Subscription for AnimationFrames {
    type Msg = f64;
    type Handle = Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<f64>,
    {
        let sender = mailbox.sender();

        let scheduler = Rc::new(Scheduler {
            window: web::window().ok_or("no global `Window` exists")?,
            running: Cell::new(true),
            current_id: Cell::new(None),
        });
        let closure = Rc::new(OnceCell::<Closure<dyn Fn(f64)>>::new());

        let scheduler2 = scheduler.clone();
        let closure2 = Cell::new(Some(closure.clone()));

        closure
            .set(Closure::wrap(Box::new(move |timestamp| {
                // Take the closure instance to prevent circular references.
                let closure = closure2.take();

                if scheduler2.running.get() {
                    sender.send_message(timestamp);

                    scheduler2.schedule(
                        closure
                            .as_ref() // Option<&Rc>
                            .unwrap_throw() // Rc<OnceCell>
                            .get() // Option<&Closure>
                            .expect_throw("closure is not set"), // &Closure
                    );

                    // Save the closure again to ensure that it is live
                    // when the next event is invoked.
                    closure2.set(closure);
                }
            }) as Box<dyn Fn(f64)>))
            .expect_throw("closure has already been set");

        scheduler.schedule(closure.get().expect_throw("closure is not set"));

        Ok(Handle { scheduler, closure })
    }
}

struct Scheduler {
    window: web::Window,
    running: Cell<bool>,
    current_id: Cell<Option<i32>>,
}

impl Scheduler {
    fn schedule(&self, closure: &Closure<dyn Fn(f64)>) {
        let id = self
            .window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap_throw();
        self.current_id.replace(Some(id));
    }

    fn cancel(&self, closure: &Closure<dyn Fn(f64)>) -> Result<(), JsValue> {
        if let Some(id) = self.current_id.take() {
            self.window.cancel_animation_frame(id)?;
        }

        self.running.set(false);

        let f: &js_sys::Function = closure.as_ref().unchecked_ref();
        f.call1(&JsValue::NULL, &0.0.into())?;

        Ok(())
    }
}

pub struct Handle {
    scheduler: Rc<Scheduler>,
    closure: Rc<OnceCell<Closure<dyn Fn(f64)>>>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        if let Some(closure) = self.closure.get() {
            let _ = self.scheduler.cancel(closure);
        }
    }
}
