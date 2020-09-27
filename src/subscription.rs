mod animation_frames;
mod interval;

pub use animation_frames::animation_frames;
pub use interval::interval;

use crate::mailbox::Mailbox;
use wasm_bindgen::prelude::*;

pub trait Subscription<TMsg> {
    type Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<TMsg>;
}
