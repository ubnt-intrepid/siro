use super::Subscription;
use crate::env::Env;
use futures::prelude::*;
use futures::{
    channel::mpsc,
    stream::{FusedStream, Stream},
    task::{self, Poll},
};
use once_cell::unsync::OnceCell;
use std::{cell::Cell, pin::Pin, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast as _};

#[inline]
pub fn animation_frames() -> AnimationFrames {
    AnimationFrames { _p: () }
}

pub struct AnimationFrames {
    _p: (),
}

impl Subscription for AnimationFrames {
    type Msg = f64;
    type Stream = AnimationFramesStream;

    fn subscribe(self, env: &Env) -> Result<Self::Stream, JsValue> {
        let (tx, rx) = mpsc::unbounded();

        let scheduler = Rc::new(Scheduler {
            window: env.window.clone(),
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
                    tx.unbounded_send(timestamp).unwrap_throw();

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

        Ok(AnimationFramesStream {
            rx,
            scheduler,
            closure,
        })
    }
}

pub struct AnimationFramesStream {
    rx: mpsc::UnboundedReceiver<f64>,
    scheduler: Rc<Scheduler>,
    closure: Rc<OnceCell<Closure<dyn Fn(f64)>>>,
}

impl AnimationFramesStream {
    fn unsubscribe(&mut self) -> Result<(), JsValue> {
        if let Some(closure) = self.closure.get() {
            let _ = self.scheduler.cancel(closure);
        }
        Ok(())
    }
}

impl Drop for AnimationFramesStream {
    fn drop(&mut self) {
        let _ = self.unsubscribe();
    }
}

impl Stream for AnimationFramesStream {
    type Item = f64;

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().rx.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rx.size_hint()
    }
}

impl FusedStream for AnimationFramesStream {
    fn is_terminated(&self) -> bool {
        self.rx.is_terminated()
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
