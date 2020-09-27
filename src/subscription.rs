mod animation_frames;
mod interval;
mod window_event;

pub use animation_frames::animation_frames;
pub use interval::interval;
pub use window_event::window_event;

use crate::mailbox::Mailbox;
use wasm_bindgen::prelude::*;

pub trait Subscription<TMsg> {
    type Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<TMsg>;
}
