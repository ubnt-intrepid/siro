use super::{text::text, ModifyView, View};
use crate::{mailbox::Mailbox, vdom::VNode};

pub fn child<C>(child: C) -> Child<C>
where
    C: View,
{
    Child { child }
}

pub struct Child<C> {
    child: C,
}

impl<TMsg: 'static, C> ModifyView<TMsg> for Child<C>
where
    C: View<Msg = TMsg>,
{
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.children.push(self.child.render(mailbox));
        }
    }
}

// ====

impl<TMsg: 'static, C> ModifyView<TMsg> for C
where
    C: View<Msg = TMsg>,
{
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.children.push(self.render(mailbox));
        }
    }
}

impl<TMsg: 'static> ModifyView<TMsg> for &'static str {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.children.push(text(self).render(mailbox));
        }
    }
}

impl<TMsg: 'static> ModifyView<TMsg> for String {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element.children.push(text(self).render(mailbox));
        }
    }
}
