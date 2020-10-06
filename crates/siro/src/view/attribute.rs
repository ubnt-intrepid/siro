use super::ModifyView;
use crate::{
    mailbox::Mailbox,
    vdom::{self, CowStr, VNode},
};

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

impl<TMsg: 'static> ModifyView<TMsg> for Attribute {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.attributes.insert(self.name, self.value);
        }
    }
}
