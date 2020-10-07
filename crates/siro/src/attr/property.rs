use super::Attr;
use crate::{
    mailbox::Mailbox,
    vdom::{self, CowStr, VElement},
};

/// Create an `Attr` that specifies an arbitrary property value, like `domNode.name = value`.
pub fn property(name: impl Into<CowStr>, value: impl Into<vdom::Property>) -> Property {
    Property {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Property {
    name: CowStr,
    value: vdom::Property,
}

impl<TMsg: 'static> Attr<TMsg> for Property {
    fn apply<M: ?Sized>(self, element: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.properties.insert(self.name, self.value);
    }
}
