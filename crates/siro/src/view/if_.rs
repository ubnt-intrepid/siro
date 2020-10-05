use super::{ModifyView, View, ViewExt as _};
use crate::{mailbox::Mailbox, vdom::VNode};

pub fn if_<M>(pred: bool, m: M) -> If<M> {
    If { pred, m }
}

pub struct If<M> {
    pred: bool,
    m: M,
}

impl<TView, M, TMsg> ModifyView<TView> for If<M>
where
    TView: View<Msg = TMsg>,
    M: ModifyView<TView, Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = M::Msg;
    type View = WithIf<M::View, TView>;

    fn modify(self, view: TView) -> Self::View {
        if self.pred {
            WithIf::Modified(view.with(self.m))
        } else {
            WithIf::NotModified(view)
        }
    }
}

pub enum WithIf<TView, UView> {
    Modified(TView),
    NotModified(UView),
}

impl<TView, UView, TMsg> View for WithIf<TView, UView>
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
            Self::Modified(l) => l.render(mailbox),
            Self::NotModified(r) => r.render(mailbox),
        }
    }
}
