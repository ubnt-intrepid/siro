use super::ModifyView;
use crate::{mailbox::Mailbox, vdom::VNode};

pub fn if_<M>(pred: bool, m: M) -> If<M> {
    If { pred, m }
}

pub struct If<M> {
    pred: bool,
    m: M,
}

impl<TMsg: 'static, T> ModifyView<TMsg> for If<T>
where
    T: ModifyView<TMsg>,
{
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if self.pred {
            self.m.modify(vnode, mailbox);
        }
    }
}
