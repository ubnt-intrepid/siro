use super::ModifyView;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VNode},
};

pub fn style(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Style {
    Style {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Style {
    name: CowStr,
    value: CowStr,
}

impl<TMsg: 'static> ModifyView<TMsg> for Style {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.styles.insert(self.name, self.value);
        }
    }
}
