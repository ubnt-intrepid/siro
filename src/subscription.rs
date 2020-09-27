mod animation_frames;
mod interval;
mod map;
mod window_event;

pub use animation_frames::{animation_frames, AnimationFrames};
pub use interval::{interval, Interval};
pub use map::Map;
pub use window_event::{window_event, WindowEvent};

use crate::mailbox::Mailbox;
use wasm_bindgen::prelude::*;

pub trait Subscription {
    type Msg: 'static;
    type Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<Msg = Self::Msg>;

    fn map<F, TMsg>(self, f: F) -> Map<Self, F, TMsg>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
        TMsg: 'static,
    {
        Map::new(self, f)
    }
}
