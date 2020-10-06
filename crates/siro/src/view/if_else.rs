use super::ModifyView;
use crate::{mailbox::Mailbox, vdom::VNode};

pub fn if_else<M1, M2>(pred: bool, m1: M1, m2: M2) -> IfElse<M1, M2> {
    IfElse { pred, m1, m2 }
}

pub struct IfElse<M1, M2> {
    pred: bool,
    m1: M1,
    m2: M2,
}

impl<TMsg: 'static, M1, M2> ModifyView<TMsg> for IfElse<M1, M2>
where
    M1: ModifyView<TMsg>,
    M2: ModifyView<TMsg>,
{
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if self.pred {
            self.m1.modify(vnode, mailbox);
        } else {
            self.m2.modify(vnode, mailbox);
        }
    }
}
