use super::{Mailbox, Sender};
use std::marker::PhantomData;

/// Create a proxy `Mailbox` to receive other type of messages.
pub fn proxy<TMsg: 'static, UMsg: 'static>(
    mailbox: impl Mailbox<TMsg>,
    f: impl Fn(UMsg) -> TMsg + Clone + 'static,
) -> impl Mailbox<UMsg> {
    Proxy { mailbox, f }
}

struct Proxy<M, F> {
    mailbox: M,
    f: F,
}

impl<M, F, TMsg, UMsg> Mailbox<TMsg> for Proxy<M, F>
where
    M: Mailbox<UMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    type Sender = ProxySender<M::Sender, F, TMsg, UMsg>;

    #[inline]
    fn send_message(&self, msg: TMsg) {
        self.mailbox.send_message((self.f)(msg));
    }

    fn sender(&self) -> Self::Sender {
        ProxySender {
            sender: self.mailbox.sender(),
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

struct ProxySender<S, F, TMsg, UMsg>
where
    S: Sender<UMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    sender: S,
    f: F,
    _marker: PhantomData<fn(TMsg) -> UMsg>,
}

impl<S, F, TMsg, UMsg> Sender<TMsg> for ProxySender<S, F, TMsg, UMsg>
where
    S: Sender<UMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    fn send_message(&self, msg: TMsg) {
        self.sender.send_message((self.f)(msg));
    }
}
