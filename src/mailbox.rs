use crate::vdom::Listener;
use futures::{channel::mpsc, prelude::*};
use gloo_events::EventListener;
use std::rc::Rc;
use web_sys as web;

pub fn mailbox<TMsg>() -> (Mailbox<TMsg>, Mails<TMsg>) {
    let (tx, rx) = mpsc::unbounded();
    (Mailbox { tx }, Mails { rx })
}

pub struct Mailbox<TMsg> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg> Clone for Mailbox<TMsg> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<TMsg> Mailbox<TMsg> {
    pub fn send(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }

    pub fn on<F>(&self, event_type: &'static str, f: F) -> Rc<dyn Listener>
    where
        TMsg: 'static,
        F: Fn(&web::Event) -> TMsg + 'static,
    {
        struct On<TMsg, F> {
            mailbox: Mailbox<TMsg>,
            event_type: &'static str,
            f: F,
        }

        impl<TMsg: 'static, F> Listener for On<TMsg, F>
        where
            F: Fn(&web_sys::Event) -> TMsg + 'static,
        {
            fn event_type(&self) -> &str {
                self.event_type
            }

            fn attach(self: Rc<Self>, target: &web::EventTarget) -> EventListener {
                EventListener::new(target, self.event_type, move |e| {
                    self.mailbox.send((self.f)(e));
                })
            }
        }

        Rc::new(On {
            mailbox: self.clone(),
            event_type,
            f,
        })
    }
}

pub struct Mails<TMsg> {
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg> Mails<TMsg> {
    pub async fn recv(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }
}
