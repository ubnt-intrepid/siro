use super::{
    text::{text, Text},
    ModifyView, View, ViewExt as _,
};
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

impl<C, P, TMsg> ModifyView<P> for Child<C>
where
    P: View<Msg = TMsg>,
    C: View<Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TMsg;
    type View = WithChild<P, C>;

    fn modify(self, parent: P) -> Self::View {
        WithChild {
            parent,
            child: self.child,
        }
    }
}

pub struct WithChild<P, C> {
    parent: P,
    child: C,
}

impl<P, C, TMsg> View for WithChild<P, C>
where
    P: View<Msg = TMsg>,
    C: View<Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        match self.parent.render(mailbox) {
            VNode::Element(mut element) => {
                element.children.push(self.child.render(mailbox));
                element.into()
            }
            node => node,
        }
    }
}

// ====

impl<P, C, TMsg> ModifyView<P> for C
where
    P: View<Msg = TMsg>,
    C: View<Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TMsg;
    type View = WithChild<P, C>;

    fn modify(self, parent: P) -> Self::View {
        parent.with(child(self))
    }
}

impl<TView> ModifyView<TView> for &'static str
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = WithChild<TView, Text<TView::Msg>>;

    fn modify(self, view: TView) -> Self::View {
        view.with(child(text(self)))
    }
}

impl<TView> ModifyView<TView> for String
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = WithChild<TView, Text<TView::Msg>>;

    fn modify(self, view: TView) -> Self::View {
        view.with(child(text(self)))
    }
}
