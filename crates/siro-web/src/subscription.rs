mod animation_frames;
mod interval;
mod map;
mod window_event;

pub use animation_frames::{animation_frames, AnimationFrames};
pub use interval::{interval, Interval};
pub use map::Map;
pub use window_event::{window_event, WindowEvent};

use crate::env::Env;
use futures::stream::FusedStream;

pub trait Subscription {
    type Msg: 'static;
    type Stream: FusedStream<Item = Self::Msg>;

    fn subscribe(self, env: &Env) -> crate::Result<Self::Stream>;

    /// Map the message type to another one.
    fn map<F, TMsg>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Msg) -> TMsg,
        TMsg: 'static,
    {
        Map::new(self, f)
    }
}
