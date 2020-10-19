mod map;

pub use map::Map;

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
