use super::{Emitter, Event};
use crate::vdom::Element;

/// Wrap an `Event` to call [`Event.preventDefault`](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)
/// before the message is emitted.
#[inline]
pub fn prevent_default<E>(event: E) -> PreventDefault<E> {
    PreventDefault { event }
}

pub struct PreventDefault<E> {
    event: E,
}

impl<T, E> Event<T> for PreventDefault<E>
where
    T: Element,
    E: Event<T>,
{
    type Msg = E::Msg;
    type Emitter = PreventDefaultEmitter<E::Emitter>;

    fn event_type(&self) -> &'static str {
        self.event.event_type()
    }

    fn into_emitter(self) -> Self::Emitter {
        PreventDefaultEmitter {
            emitter: self.event.into_emitter(),
        }
    }
}

pub struct PreventDefaultEmitter<E> {
    emitter: E,
}

impl<E> Emitter for PreventDefaultEmitter<E>
where
    E: Emitter,
{
    type Msg = E::Msg;

    fn emit(&self, event: &web::Event) -> Option<Self::Msg> {
        event.prevent_default();
        self.emitter.emit(event)
    }
}
