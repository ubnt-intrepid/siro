use super::ModifyView;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VNode},
};

pub fn class(class_name: impl Into<CowStr>) -> Class {
    Class {
        class_name: class_name.into(),
    }
}

pub struct Class {
    class_name: CowStr,
}

impl<TMsg: 'static> ModifyView<TMsg> for Class {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.classes.insert(self.class_name);
        }
    }
}
