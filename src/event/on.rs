use super::{EventHandler, EventHandlerBase};
use crate::element::Element;

pub fn on<F, TMsg>(event_type: &'static str, f: F) -> On<F>
where
    F: Fn(&web::Event) -> TMsg,
    TMsg: 'static,
{
    On { event_type, f }
}

pub struct On<F> {
    event_type: &'static str,
    f: F,
}

impl<F, TMsg> EventHandlerBase for On<F>
where
    F: Fn(&web::Event) -> TMsg,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn event_type(&self) -> &'static str {
        self.event_type
    }

    fn invoke(&self, event: &web::Event) -> Option<Self::Msg> {
        Some((self.f)(event))
    }
}

impl<T, F, TMsg> EventHandler<T> for On<F>
where
    T: Element,
    F: Fn(&web::Event) -> TMsg,
    TMsg: 'static,
{
}
