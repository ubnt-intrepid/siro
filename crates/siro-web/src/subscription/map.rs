use super::Subscription;
use crate::env::Env;
use futures::prelude::*;
use wasm_bindgen::JsValue;

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
    F: FnMut(S::Msg) -> TMsg,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Stream = futures::stream::Map<S::Stream, F>;

    fn subscribe(self, env: &Env) -> Result<Self::Stream, JsValue> {
        Ok(self.subscription.subscribe(env)?.map(self.f))
    }
}
