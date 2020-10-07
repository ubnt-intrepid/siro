use super::Attr;
use crate::{
    mailbox::Mailbox,
    vdom::{self, CowStr, VElement},
};

/// Create an `Attr` that specifies an arbitrary attribute value, like `domNode.setAttribute(name, value)`.
pub fn attribute(name: impl Into<CowStr>, value: impl Into<vdom::Attribute>) -> Attribute {
    Attribute {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Attribute {
    name: CowStr,
    value: vdom::Attribute,
}

impl<TMsg: 'static> Attr<TMsg> for Attribute {
    fn apply<M: ?Sized>(self, element: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.attributes.insert(self.name, self.value);
    }
}
