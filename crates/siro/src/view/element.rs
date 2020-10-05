use super::View;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VElement, VNode},
};
use std::marker::PhantomData;

pub fn element<TMsg: 'static>(
    tag_name: impl Into<CowStr>,
    namespace_uri: Option<CowStr>,
) -> Element<TMsg> {
    Element {
        tag_name: tag_name.into(),
        namespace_uri,
        _marker: PhantomData,
    }
}

pub struct Element<TMsg> {
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static> View for Element<TMsg> {
    type Msg = TMsg;

    fn render<M: ?Sized>(self, _: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        VNode::Element(VElement::new(
            self.tag_name.clone(),
            self.namespace_uri.clone(),
        ))
    }
}
