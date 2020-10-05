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

impl<TView, I, TMsg> ModifyView<TView> for Iter<I>
where
    TView: View<Msg = TMsg>,
    I: IntoIterator,
    I::Item: View<Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TView::Msg;
    type View = WithIter<TView, I>;

    fn modify(self, view: TView) -> Self::View {
        WithIter {
            view,
            iter: self.iter,
        }
    }
}

pub struct WithIter<TView, I> {
    view: TView,
    iter: I,
}

impl<TView, I, TMsg> View for WithIter<TView, I>
where
    TView: View<Msg = TMsg>,
    I: IntoIterator,
    I::Item: View<Msg = TMsg>,
    TMsg: 'static,
{
    type Msg = TView::Msg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        match self.view.render(mailbox) {
            VNode::Element(mut element) => {
                element
                    .children
                    .extend(self.iter.into_iter().map(|view| view.render(mailbox)));
                element.into()
            }
            node => node,
        }
    }
}
