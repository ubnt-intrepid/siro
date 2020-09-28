mod on;
mod on_input;

pub use on::{on, On};
pub use on_input::{on_input, HasInputEvent, OnInput};

use crate::builder::Element;

pub trait EventHandlerBase {
    type Msg: 'static;

    fn event_type(&self) -> &'static str;
    fn invoke(&self, event: &web::Event) -> Option<Self::Msg>;
}

pub trait EventHandler<E: Element>: EventHandlerBase {}
