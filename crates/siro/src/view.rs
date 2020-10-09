mod children;
mod element;
mod text;

pub use children::{iter, Children, Iter};
pub use element::{element, Element};
pub use text::{text, Text};

use crate::{
    attr::Attr,
    mailbox::{Mailbox, MailboxExt as _},
    vdom::VNode,
};

/// The view object that renders virtual DOM.
pub trait View {
    /// The message type associated with this view.
    type Msg: 'static;

    /// Render the virtual DOM.
    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>;
}

pub trait ViewExt: View {
    fn attr<A>(self, attr: A) -> WithAttr<Self, A>
    where
        Self: Sized,
        A: Attr<Self::Msg>,
    {
        WithAttr { view: self, attr }
    }

    fn children<C>(self, children: C) -> WithChildren<Self, C>
    where
        Self: Sized,
        C: Children<Self::Msg>,
    {
        WithChildren {
            view: self,
            children,
        }
    }

    fn map<F, TMsg: 'static>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
    {
        Map { view: self, f }
    }
}

impl<TView> ViewExt for TView where TView: View {}

// ==== WithAttr ====

pub struct WithAttr<TView, A> {
    view: TView,
    attr: A,
}

impl<TView, A> View for WithAttr<TView, A>
where
    TView: View,
    A: Attr<TView::Msg>,
{
    type Msg = TView::Msg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        let mut vnode = self.view.render(mailbox);
        if let VNode::Element(element) = &mut vnode {
            self.attr.apply(element, mailbox);
        }
        vnode
    }
}

// ==== WithChildren ====

pub struct WithChildren<TView, C> {
    view: TView,
    children: C,
}

impl<TView, C> View for WithChildren<TView, C>
where
    TView: View,
    C: Children<TView::Msg>,
{
    type Msg = TView::Msg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        let mut vnode = self.view.render(mailbox);
        if let VNode::Element(element) = &mut vnode {
            self.children.append(element, mailbox);
        }
        vnode
    }
}

// ==== Map ====

pub struct Map<TView, F> {
    view: TView,
    f: F,
}

impl<TView, F, TMsg> View for Map<TView, F>
where
    TView: View,
    F: Fn(TView::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        self.view.render(&mailbox.map(self.f))
    }
}
