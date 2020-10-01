use super::{Emitter, Event};
use crate::vdom::Element;

/// Create an `Event` corresponding to the provided event type.
#[inline]
pub fn on_<F, TMsg>(event_type: &'static str, f: F) -> OnOpt<F>
where
    F: Fn(&web::Event) -> Option<TMsg> + 'static,
    TMsg: 'static,
{
    OnOpt { event_type, f }
}

pub struct OnOpt<F> {
    event_type: &'static str,
    f: F,
}

impl<T, F, TMsg> Event<T> for OnOpt<F>
where
    T: Element,
    F: Fn(&web::Event) ->Option<TMsg> + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Emitter = OnOptEmitter<F>;

    fn event_type(&self) -> &'static str {
        self.event_type
    }

    fn into_emitter(self) -> Self::Emitter {
        OnOptEmitter { f: self.f }
    }
}

pub struct OnOptEmitter<F> {
    f: F,
}

impl<F, TMsg> Emitter for OnOptEmitter<F>
where
    F: Fn(&web::Event) -> Option<TMsg> + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn emit(&self, event: &web::Event) -> Option<Self::Msg> {
        (self.f)(event)
    }
}
