use super::Attr;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VElement},
};

/// Create an `Attr` that specifies the inner HTML content of the element.
pub fn inner_html(inner_html: impl Into<CowStr>) -> InnerHtml {
    InnerHtml {
        inner_html: inner_html.into(),
    }
}

pub struct InnerHtml {
    inner_html: CowStr,
}

impl<TMsg: 'static> Attr<TMsg> for InnerHtml {
    fn apply<M: ?Sized>(self, element: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.inner_html = Some(self.inner_html);
    }
}
