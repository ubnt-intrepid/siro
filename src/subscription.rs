mod animation_frames;
mod interval;

pub use animation_frames::animation_frames;
pub use interval::interval;

use crate::mailbox::Mailbox;
use wasm_bindgen::prelude::*;

pub trait Subscription<TMsg> {
    type Handle;

    fn subscribe(
        self,
        window: &web::Window,
        mailbox: impl Mailbox<TMsg> + 'static,
    ) -> Result<Self::Handle, JsValue>;
}
