use super::Subscribe;
use crate::mailbox::Mailbox;
use std::marker::PhantomData;

pub struct Map<S, F, TMsg> {
    subscribe: S,
    f: F,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<S, F, TMsg> Map<S, F, TMsg> {
    pub(super) fn new(subscribe: S, f: F) -> Self {
        Self {
            subscribe,
            f,
            _marker: PhantomData,
        }
    }
}

impl<S, F, TMsg> Subscribe for Map<S, F, TMsg>
where
    S: Subscribe,
    S::Msg: 'static,
    F: Fn(S::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Error = S::Error;
    type Subscription = S::Subscription;

    fn subscribe<M: ?Sized>(self, mailbox: &M) -> Result<Self::Subscription, Self::Error>
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        self.subscribe.subscribe(&mailbox.map(self.f))
    }
}
