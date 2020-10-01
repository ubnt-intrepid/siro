mod on;
pub use on::{on, On};

use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{Element, Listener},
};
use gloo_events::EventListener;
use std::rc::Rc;

pub trait EventHandlerBase {
    type Msg: 'static;

    fn event_type(&self) -> &'static str;
    fn invoke(&self, event: &web::Event) -> Option<Self::Msg>;
}

pub trait EventHandler<E: Element>: EventHandlerBase {}

pub trait ElementEventExt: Element {
    fn event<M, E>(self, mailbox: M, handler: E) -> Self
    where
        M: Mailbox,
        E: EventHandler<Self, Msg = M::Msg> + 'static,
    {
        self.listener(Box::new(EventHandlerListener(Rc::new(Inner {
            sender: mailbox.sender(),
            handler,
        }))))
    }
}

impl<E> ElementEventExt for E where E: Element {}

struct EventHandlerListener<S, E>(Rc<Inner<S, E>>);

struct Inner<S, E> {
    sender: S,
    handler: E,
}

impl<S, E> Listener for EventHandlerListener<S, E>
where
    S: Sender,
    E: EventHandlerBase<Msg = S::Msg> + 'static,
{
    fn event_type(&self) -> &'static str {
        self.0.handler.event_type()
    }

    fn attach(&self, target: &web::EventTarget) -> EventListener {
        let inner = self.0.clone();
        EventListener::new(target, self.event_type(), move |e| {
            if let Some(msg) = inner.handler.invoke(e) {
                inner.sender.send_message(msg);
            }
        })
    }
}
