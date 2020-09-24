use super::Subscription;
use crate::mailbox::Mailbox;
use once_cell::unsync::OnceCell;
use std::{any::Any, cell::Cell, mem, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast as _};

pub fn animation_frame<F, TMsg>(callback: F) -> impl Subscription<TMsg>
where
    F: Fn(f64) -> TMsg + 'static,
{
    FramesSubscription { callback }
}

struct State {
    window: web::Window,
    running: Cell<bool>,
    current_id: Cell<Option<i32>>,
}

impl State {
    fn request_frame(&self, closure: &Closure<dyn Fn(f64)>) {
        let id = self
            .window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .unwrap_throw();
        self.current_id.replace(Some(id));
    }

    fn cancel(&self) {
        if let Some(id) = self.current_id.take() {
            self.window.cancel_animation_frame(id).unwrap_throw();
        }
    }
}

struct CancelOnDrop(Rc<State>);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

struct FramesSubscription<F> {
    callback: F,
}

impl<F, TMsg> Subscription<TMsg> for FramesSubscription<F>
where
    F: Fn(f64) -> TMsg + 'static,
{
    fn subscribe(
        self,
        window: &web::Window,
        mailbox: impl Mailbox<TMsg> + 'static,
    ) -> Result<Box<dyn Any>, JsValue> {
        let Self { callback } = self;

        let closure = Rc::new(OnceCell::<Closure<dyn Fn(f64)>>::new());
        let state = Rc::new(State {
            window: window.clone(),
            running: Cell::new(true),
            current_id: Cell::new(None),
        });

        let closure2 = closure.clone();
        let state2 = state.clone();

        closure
            .set(Closure::wrap(Box::new(move |timestamp| {
                mailbox.send_message(callback(timestamp));

                if state2.running.get() {
                    let closure = closure2.get().expect_throw("closure is not set");
                    state2.request_frame(&closure);
                }
            }) as Box<dyn Fn(f64)>))
            .expect_throw("closure has already been set");

        state.request_frame(closure.get().expect_throw("closure is not set"));

        // This leakage is intentional to ensure that the closure passed to request_animation_frame()
        // is not dropped when the callback is invoked.
        mem::forget(closure);

        Ok(Box::new(CancelOnDrop(state)))
    }
}
