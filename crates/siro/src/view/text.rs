use super::View;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VNode, VText},
};
use std::marker::PhantomData;

pub fn text<TMsg: 'static>(content: impl Into<CowStr>) -> Text<TMsg> {
    Text {
        content: content.into(),
        _marker: PhantomData,
    }
}

pub struct Text<TMsg> {
    content: CowStr,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static> View for Text<TMsg> {
    type Msg = TMsg;

    fn render<M: ?Sized>(self, _: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        VNode::Text(VText::new(self.content))
    }
}
