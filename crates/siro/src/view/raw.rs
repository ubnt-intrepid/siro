use super::View;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, CustomNode, VNode},
};
use std::marker::PhantomData;

pub fn raw<TMsg: 'static>(inner_html: impl Into<CowStr>) -> Raw<TMsg> {
    Raw {
        inner_html: inner_html.into(),
        _marker: PhantomData,
    }
}

pub struct Raw<TMsg> {
    inner_html: CowStr,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static> View for Raw<TMsg> {
    type Msg = TMsg;

    fn render<M: ?Sized>(self, _: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        VNode::Custom(CustomNode::new(move |document| {
            let node = document.create_element("div")?;
            node.set_inner_html(&*self.inner_html);
            Ok(node.into())
        }))
    }
}
