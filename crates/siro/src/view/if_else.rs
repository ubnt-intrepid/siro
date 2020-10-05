use super::{ModifyView, View, ViewExt as _};
use crate::{mailbox::Mailbox, vdom::VNode};

pub fn if_else<M1, M2>(pred: bool, m1: M1, m2: M2) -> IfElse<M1, M2> {
    IfElse { pred, m1, m2 }
}

pub struct IfElse<M1, M2> {
    pred: bool,
    m1: M1,
    m2: M2,
}

impl<TView, M1, M2, TMsg> ModifyView<TView> for IfElse<M1, M2>
where
    TView: View<Msg = TMsg>,
    M1: ModifyView<TView, Msg = TMsg>,
    M2: ModifyView<TView, Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TMsg;
    type View = WithIfElse<M1::View, M2::View>;

    fn modify(self, view: TView) -> Self::View {
        if self.pred {
            WithIfElse::True(view.with(self.m1))
        } else {
            WithIfElse::False(view.with(self.m2))
        }
    }
}

pub enum WithIfElse<TView, UView> {
    True(TView),
    False(UView),
}

impl<TView, UView, TMsg> View for WithIfElse<TView, UView>
where
    TView: View<Msg = TMsg>,
    UView: View<Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        match self {
            Self::True(l) => l.render(mailbox),
            Self::False(r) => r.render(mailbox),
        }
    }
}
