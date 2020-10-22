use super::{Mailbox, Subscriber, Subscription};
use std::marker::PhantomData;

pub struct Map<S, F> {
    subscription: S,
    f: F,
}

impl<S, F> Map<S, F> {
    pub(super) fn new(subscription: S, f: F) -> Self {
        Self { subscription, f }
    }
}

impl<S, F, TMsg> Subscription for Map<S, F>
where
    S: Subscription,
    F: Fn(S::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Error = S::Error;
    type Subscribe = S::Subscribe;

    fn subscribe<T>(self, subscriber: T) -> Result<Self::Subscribe, Self::Error>
    where
        T: Subscriber<Msg = Self::Msg>,
    {
        self.subscription.subscribe(MapSubscriber {
            subscriber,
            f: self.f,
            _marker: PhantomData,
        })
    }
}

struct MapSubscriber<S, F, TMsg> {
    subscriber: S,
    f: F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<S, F, TMsg> Subscriber for MapSubscriber<S, F, TMsg>
where
    S: Subscriber,
    F: Fn(TMsg) -> S::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Mailbox = MapMailbox<S::Mailbox, F, TMsg>;

    fn mailbox(&self) -> Self::Mailbox {
        MapMailbox {
            mailbox: self.subscriber.mailbox(),
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

struct MapMailbox<M, F, TMsg> {
    mailbox: M,
    f: F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<M, F, TMsg> Mailbox for MapMailbox<M, F, TMsg>
where
    M: Mailbox,
    F: Fn(TMsg) -> M::Msg + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn send_message(&self, msg: Self::Msg) {
        self.mailbox.send_message((self.f)(msg));
    }
}
