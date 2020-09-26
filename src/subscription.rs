mod animation_frames;
mod interval;

pub use animation_frames::animation_frames;
pub use interval::interval;

use crate::mailbox::Sender;
use wasm_bindgen::prelude::*;

pub trait Subscription<TMsg> {
    type Handle;

    fn subscribe<TSender>(self, sender: TSender) -> Result<Self::Handle, JsValue>
    where
        TSender: Sender<TMsg>;
}
