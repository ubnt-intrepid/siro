use crate::vdom::Listener;
use futures::channel::mpsc;
use gloo_events::EventListener;
use std::rc::Rc;
use web_sys as web;

pub struct Mailbox<TMsg: 'static>(mpsc::UnboundedSender<TMsg>);

impl<TMsg: 'static> Mailbox<TMsg> {
    pub fn pair() -> (Self, mpsc::UnboundedReceiver<TMsg>) {
        let (tx, rx) = mpsc::unbounded();
        (Self(tx), rx)
    }

    pub fn on_click(&self, f: impl Fn(&web_sys::Event) -> TMsg + 'static) -> Rc<dyn Listener> {
        struct OnClick<T, F> {
            tx: mpsc::UnboundedSender<T>,
            f: F,
        }

        impl<TMsg: 'static, F> Listener for OnClick<TMsg, F>
        where
            F: Fn(&web_sys::Event) -> TMsg + 'static,
        {
            fn event_type(&self) -> &str {
                "click"
            }

            fn attach(self: Rc<Self>, target: &web::EventTarget) -> EventListener {
                EventListener::new(target, "click", move |e| {
                    let msg = (self.f)(e);
                    let _ = self.tx.unbounded_send(msg);
                })
            }
        }

        Rc::new(OnClick {
            tx: self.0.clone(),
            f,
        })
    }
}
