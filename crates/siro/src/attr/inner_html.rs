use super::Attr;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VElement},
};
use std::marker::PhantomData;

/// Create an `Attr` that specifies the inner HTML content of the element.
pub fn inner_html<TMsg: 'static>(inner_html: impl Into<CowStr>) -> InnerHtml<TMsg> {
    InnerHtml {
        inner_html: inner_html.into(),
        _marker: PhantomData,
    }
}

pub struct InnerHtml<TMsg> {
    inner_html: CowStr,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static> Attr<TMsg> for InnerHtml<TMsg> {
    fn apply<M: ?Sized>(self, element: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.inner_html = Some(self.inner_html);
    }
}
