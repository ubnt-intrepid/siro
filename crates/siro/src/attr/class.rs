use super::Attr;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VElement},
};

/// Create an `Attr` that specify a CSS class name.
pub fn class(class_name: impl Into<CowStr>) -> Class {
    Class {
        class_name: class_name.into(),
    }
}

pub struct Class {
    class_name: CowStr,
}

impl<TMsg: 'static> Attr<TMsg> for Class {
    fn apply<M: ?Sized>(self, element: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.classes.insert(self.class_name);
    }
}
