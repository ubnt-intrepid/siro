mod attribute;
mod child;
mod class;
mod element;
mod if_;
mod if_else;
mod iter;
mod property;
mod raw;
mod style;
mod text;

pub use attribute::{attribute, Attribute};
pub use child::{child, Child};
pub use class::{class, Class};
pub use element::{element, Element};
pub use if_::{if_, If};
pub use if_else::{if_else, IfElse};
pub use iter::{iter, Iter};
pub use property::{property, Property};
pub use raw::{raw, Raw};
pub use style::{style, Style};
pub use text::{text, Text};

use crate::{mailbox::Mailbox, vdom::VNode};

/// The view object that renders virtual DOM.
pub trait View {
    /// The message type associated with this view.
    type Msg: 'static;

    /// Render the virtual DOM.
    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>;
}

/// The modifier of a `View`.
pub trait ModifyView<TMsg: 'static> {
    fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>;
}

impl<TMsg: 'static> ModifyView<TMsg> for () {
    fn modify<M: ?Sized>(self, _: &mut VNode, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
    }
}

macro_rules! impl_modifier_for_tuples {
    ( $H:ident, $( $T:ident ),* ) => {
        impl<TMsg: 'static, $H, $( $T ),*> ModifyView<TMsg> for ($H, $( $T ),*)
        where
            $H: ModifyView<TMsg>,
            $( $T: ModifyView<TMsg>, )*
        {
            fn modify<M:?Sized>(self, vnode: &mut VNode, mailbox: &M)
            where
                M: Mailbox<Msg = TMsg>,
            {
                #[allow(non_snake_case)]
                let ($H, $( $T ),*) = self;
                $H.modify(vnode, mailbox);
                $( $T.modify(vnode, mailbox); )*
            }
        }

        impl_modifier_for_tuples!($($T),*);
    };
    ( $T:ident ) => {
        impl<TMsg: 'static, $T> ModifyView<TMsg> for ($T,)
        where
            $T: ModifyView<TMsg>,
        {
            fn modify<M: ?Sized>(self, vnode: &mut VNode, mailbox: &M)
            where
                M: Mailbox<Msg = TMsg>,
            {
                self.0.modify(vnode, mailbox);
            }
        }
    };
}

impl_modifier_for_tuples!(M1, M2, M3, M4, M5, M6, M7, M8, M9, M10);

pub trait ViewExt: View {
    fn with<M>(self, modifier: M) -> With<Self, M>
    where
        Self: Sized,
        M: ModifyView<Self::Msg>,
    {
        With {
            view: self,
            modifier,
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

pub struct With<TView, T> {
    view: TView,
    modifier: T,
}

impl<TView, T> View for With<TView, T>
where
    TView: View,
    T: ModifyView<TView::Msg>,
{
    type Msg = TView::Msg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        let mut vnode = self.view.render(mailbox);
        self.modifier.modify(&mut vnode, mailbox);
        vnode
    }
}

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
