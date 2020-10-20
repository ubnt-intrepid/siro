use once_cell::unsync::OnceCell;
use siro::subscription::{Mailbox as _, Subscribe, Subscriber, Subscription};
use std::{cell::Cell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn animation_frames() -> impl Subscription<Msg = f64, Error = JsValue> {
    SubscribeAnimationFrames { _p: () }
}

struct SubscribeAnimationFrames {
    _p: (),
}

impl Subscription for SubscribeAnimationFrames {
    type Msg = f64;
    type Error = JsValue;
    type Subscribe = AnimationFramesSubscription;

    fn subscribe<Ctx>(self, ctx: Ctx) -> Result<Self::Subscribe, Self::Error>
    where
        Ctx: Subscriber<Msg = Self::Msg>,
    {
        let mailbox = ctx.mailbox();

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
                    mailbox.send_message(timestamp);

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

        Ok(AnimationFramesSubscription { scheduler, closure })
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

struct AnimationFramesSubscription {
    scheduler: Rc<Scheduler>,
    closure: Rc<OnceCell<Closure<dyn Fn(f64)>>>,
}

impl Subscribe for AnimationFramesSubscription {
    type Msg = ();
    type Error = JsValue;

    fn unsubscribe(&mut self) -> Result<(), Self::Error> {
        if let Some(closure) = self.closure.get() {
            let _ = self.scheduler.cancel(closure);
        }
        Ok(())
    }
}

impl Drop for AnimationFramesSubscription {
    fn drop(&mut self) {
        let _ = self.unsubscribe();
    }
}
