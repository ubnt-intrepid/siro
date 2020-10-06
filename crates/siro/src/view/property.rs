use super::ModifyView;
use crate::{
    mailbox::Mailbox,
    vdom::{self, CowStr, VNode},
};

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

impl<TMsg: 'static> ModifyView<TMsg> for Property {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.properties.insert(self.name, self.value);
        }
    }
}
