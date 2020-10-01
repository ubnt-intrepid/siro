use super::{Emitter, Event};
use crate::vdom::Element;

/// Create an `Event` corresponding to the provided event type.
#[inline]
pub fn on<F, TMsg>(event_type: &'static str, f: F) -> On<F>
where
    F: Fn(&web::Event) -> TMsg + 'static,
    TMsg: 'static,
{
    On { event_type, f }
}

pub struct On<F> {
    event_type: &'static str,
    f: F,
}

impl<T, F, TMsg> Event<T> for On<F>
where
    T: Element,
    F: Fn(&web::Event) -> TMsg + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Emitter = OnEmitter<F>;

    fn event_type(&self) -> &'static str {
        self.event_type
    }

    fn into_emitter(self) -> Self::Emitter {
        OnEmitter { f: self.f }
    }
}

pub struct OnEmitter<F> {
    f: F,
}

impl<F, TMsg> Emitter for OnEmitter<F>
where
    F: Fn(&web::Event) -> TMsg + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn emit(&self, event: &web::Event) -> Option<Self::Msg> {
        Some((self.f)(event))
    }
}
