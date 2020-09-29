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
        self.listener(Rc::new(EventHandlerListener {
            sender: mailbox.sender(),
            handler,
        }))
    }
}

impl<E> ElementEventExt for E where E: Element {}

struct EventHandlerListener<S, E> {
    sender: S,
    handler: E,
}

impl<S, E> Listener for EventHandlerListener<S, E>
where
    S: Sender + 'static,
    E: EventHandlerBase<Msg = S::Msg> + 'static,
{
    fn event_type(&self) -> &str {
        self.handler.event_type()
    }

    fn attach(self: Rc<Self>, target: &web::EventTarget) -> EventListener {
        EventListener::new(target, self.handler.event_type(), move |e| {
            if let Some(msg) = self.handler.invoke(e) {
                self.sender.send_message(msg);
            }
        })
    }
}
