use super::{Mailbox, Sender};
use std::marker::PhantomData;

pub struct Map<M, F, TMsg> {
    mailbox: M,
    f: F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<M, F, TMsg> Map<M, F, TMsg> {
    #[inline]
    pub(super) fn new(mailbox: M, f: F) -> Self {
        Self {
            mailbox,
            f,
            _marker: PhantomData,
        }
    }
}

impl<M, F, TMsg> Mailbox for Map<M, F, TMsg>
where
    M: Mailbox,
    F: Fn(TMsg) -> M::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Sender = MapSender<M::Sender, F, TMsg>;

    #[inline]
    fn send_message(&self, msg: Self::Msg) {
        self.mailbox.send_message((self.f)(msg));
    }

    fn sender(&self) -> Self::Sender {
        MapSender {
            sender: self.mailbox.sender(),
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

pub struct MapSender<S, F, TMsg> {
    sender: S,
    f: F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<S, F, TMsg> Sender for MapSender<S, F, TMsg>
where
    S: Sender,
    F: Fn(TMsg) -> S::Msg + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn send_message(&self, msg: Self::Msg) {
        self.sender.send_message((self.f)(msg));
    }
}
