mod on;

pub use on::{on, On};

use crate::builder::Element;

pub trait EventHandlerBase {
    type Msg: 'static;

    fn event_type(&self) -> &'static str;
    fn invoke(&self, event: &web::Event) -> Option<Self::Msg>;
}

pub trait EventHandler<E: Element>: EventHandlerBase {}
