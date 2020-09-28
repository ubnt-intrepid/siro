use super::{EventHandler, EventHandlerBase};
use crate::builder::Element;

pub trait HasInputEvent: Element {}

pub fn on_input<F, TMsg>(f: F) -> OnInput<F>
where
    F: Fn(String) -> TMsg,
    TMsg: 'static,
{
    OnInput { f }
}

pub struct OnInput<F> {
    f: F,
}

impl<F, TMsg> EventHandlerBase for OnInput<F>
where
    F: Fn(String) -> TMsg,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn event_type(&self) -> &'static str {
        "input"
    }

    fn invoke(&self, event: &web::Event) -> Option<Self::Msg> {
        Some((self.f)(
            js_sys::Reflect::get(&&event.target()?, &"value".into())
                .ok()?
                .as_string()?,
        ))
    }
}

impl<T, F, TMsg> EventHandler<T> for OnInput<F>
where
    T: HasInputEvent,
    F: Fn(String) -> TMsg,
    TMsg: 'static,
{
}
