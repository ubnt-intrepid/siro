use super::{ModifyView, View};
use crate::{mailbox::Mailbox, vdom::VNode};

pub fn iter<I>(iter: I) -> Iter<I>
where
    I: IntoIterator,
    I::Item: View,
{
    Iter { iter }
}

pub struct Iter<I> {
    iter: I,
}

impl<TMsg: 'static, I> ModifyView<TMsg> for Iter<I>
where
    I: IntoIterator,
    I::Item: View<Msg = TMsg>,
{
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let VNode::Element(element) = vnode {
            element
                .children
                .extend(self.iter.into_iter().map(|view| view.render(mailbox)));
        }
    }
}
