mod animation_frames;
mod interval;
mod map;
mod window_event;

pub use animation_frames::animation_frames;
pub use interval::interval;
pub use map::Map;
pub use window_event::{window_event, WindowEvent};

use crate::mailbox::Mailbox;

pub trait Subscribe {
    type Msg: 'static;
    type Error;
    type Subscription: Subscription<Error = Self::Error>;

    fn subscribe<M: ?Sized>(self, mailbox: &M) -> Result<Self::Subscription, Self::Error>
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

pub trait Subscription {
    type Msg: 'static;
    type Error;

    fn unsubscribe(&mut self) -> Result<(), Self::Error>;
}
