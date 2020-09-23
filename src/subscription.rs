mod animation_frame;
mod interval;

pub use animation_frame::animation_frame;
pub use interval::interval;

use crate::mailbox::Mailbox;
use std::any::Any;
use wasm_bindgen::prelude::*;

pub trait Subscription<TMsg> {
    fn subscribe(
        self,
        window: &web::Window,
        mailbox: impl Mailbox<TMsg> + 'static,
    ) -> Result<Box<dyn Any>, JsValue>;
}
