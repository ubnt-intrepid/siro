mod on;
mod on_;
mod prevent_default;

pub use on::{on, On};
pub use on_::{on_, OnOpt};
pub use prevent_default::{prevent_default, PreventDefault};

use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{Element, Listener},
};
use gloo_events::EventListener;
use std::rc::Rc;

/// A trait that represents events from DOM.
pub trait Event<E: Element> {
    /// The message type emitted from `Emitter`.
    type Msg: 'static;

    /// The type of `Emitter` associated with this event.
    type Emitter: Emitter<Msg = Self::Msg>;

    /// Return the name of this event.
    fn event_type(&self) -> &'static str;

    /// Convert itself into an `Emitter`.
    fn into_emitter(self) -> Self::Emitter;
}

/// The emitter of events from DOM.
pub trait Emitter: 'static {
    /// The message type.
    type Msg: 'static;

    /// Emit a message.
    fn emit(&self, event: &web::Event) -> Option<Self::Msg>;
}

/// A mix-in trait for `Element`s for specifying `Event`s.
pub trait ElementEventExt: Element {
    fn event<M, E>(self, mailbox: M, event: E) -> Self
    where
        M: Mailbox,
        E: Event<Self, Msg = M::Msg>,
    {
        self.listener(Box::new(EventHandlerListener(Rc::new(Inner {
            event_type: event.event_type(),
            sender: mailbox.sender(),
            handler: event.into_emitter(),
        }))))
    }
}

impl<E> ElementEventExt for E where E: Element {}

struct EventHandlerListener<S, E>(Rc<Inner<S, E>>);

struct Inner<S, E> {
    event_type: &'static str,
    sender: S,
    handler: E,
}

impl<S, E> Listener for EventHandlerListener<S, E>
where
    S: Sender,
    E: Emitter<Msg = S::Msg>,
{
    fn event_type(&self) -> &'static str {
        self.0.event_type
    }

    fn attach(&self, target: &web::EventTarget) -> EventListener {
        let inner = self.0.clone();
        EventListener::new(target, self.event_type(), move |e| {
            if let Some(msg) = inner.handler.emit(e) {
                inner.sender.send_message(msg);
            }
        })
    }
}
