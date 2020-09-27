use super::Subscription;
use crate::mailbox::Mailbox;
use std::marker::PhantomData;
use wasm_bindgen::prelude::*;

pub struct Map<S, F, TMsg> {
    subscription: S,
    f: F,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<S, F, TMsg> Map<S, F, TMsg> {
    pub(super) fn new(subscription: S, f: F) -> Self {
        Self {
            subscription,
            f,
            _marker: PhantomData,
        }
    }
}

impl<S, F, TMsg> Subscription for Map<S, F, TMsg>
where
    S: Subscription,
    S::Msg: 'static,
    F: Fn(S::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Handle = S::Handle;

    fn subscribe<M>(self, mailbox: &M) -> Result<Self::Handle, JsValue>
    where
        M: Mailbox<TMsg>,
    {
        let mailbox = crate::mailbox::proxy(mailbox, self.f);
        self.subscription.subscribe(&mailbox)
    }
}
