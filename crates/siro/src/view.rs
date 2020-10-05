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
pub trait ModifyView<TView: View> {
    /// The message type associated with modified view.
    type Msg: 'static;

    /// The type of modified `View`.
    type View: View<Msg = Self::Msg>;

    /// Modify a `View` into another one.
    fn modify(self, view: TView) -> Self::View;
}

impl<TView> ModifyView<TView> for ()
where
    TView: View,
{
    type Msg = TView::Msg;
    type View = TView;

    fn modify(self, view: TView) -> Self::View {
        view
    }
}

impl<TView, M> ModifyView<TView> for (M,)
where
    TView: View,
    M: ModifyView<TView>,
{
    type Msg = M::Msg;
    type View = M::View;

    fn modify(self, view: TView) -> Self::View {
        self.0.modify(view)
    }
}

impl<TView, M1, M2> ModifyView<TView> for (M1, M2)
where
    TView: View,
    M1: ModifyView<TView>,
    M2: ModifyView<M1::View>,
{
    type Msg = M2::Msg;
    type View = M2::View;

    fn modify(self, view: TView) -> Self::View {
        let view = self.0.modify(view);
        self.1.modify(view)
    }
}

impl<TView, M1, M2, M3> ModifyView<TView> for (M1, M2, M3)
where
    TView: View,
    M1: ModifyView<TView>,
    M2: ModifyView<M1::View>,
    M3: ModifyView<M2::View>,
{
    type Msg = M3::Msg;
    type View = M3::View;

    fn modify(self, view: TView) -> Self::View {
        let view = self.0.modify(view);
        let view = self.1.modify(view);
        self.2.modify(view)
    }
}

impl<TView, M1, M2, M3, M4> ModifyView<TView> for (M1, M2, M3, M4)
where
    TView: View,
    M1: ModifyView<TView>,
    M2: ModifyView<M1::View>,
    M3: ModifyView<M2::View>,
    M4: ModifyView<M3::View>,
{
    type Msg = M4::Msg;
    type View = M4::View;

    fn modify(self, view: TView) -> Self::View {
        let view = self.0.modify(view);
        let view = self.1.modify(view);
        let view = self.2.modify(view);
        self.3.modify(view)
    }
}

impl<TView, M1, M2, M3, M4, M5> ModifyView<TView> for (M1, M2, M3, M4, M5)
where
    TView: View,
    M1: ModifyView<TView>,
    M2: ModifyView<M1::View>,
    M3: ModifyView<M2::View>,
    M4: ModifyView<M3::View>,
    M5: ModifyView<M4::View>,
{
    type Msg = M5::Msg;
    type View = M5::View;

    fn modify(self, view: TView) -> Self::View {
        let view = self.0.modify(view);
        let view = self.1.modify(view);
        let view = self.2.modify(view);
        let view = self.3.modify(view);
        self.4.modify(view)
    }
}

impl<TView, M1, M2, M3, M4, M5, M6> ModifyView<TView> for (M1, M2, M3, M4, M5, M6)
where
    TView: View,
    M1: ModifyView<TView>,
    M2: ModifyView<M1::View>,
    M3: ModifyView<M2::View>,
    M4: ModifyView<M3::View>,
    M5: ModifyView<M4::View>,
    M6: ModifyView<M5::View>,
{
    type Msg = M6::Msg;
    type View = M6::View;

    fn modify(self, view: TView) -> Self::View {
        let view = self.0.modify(view);
        let view = self.1.modify(view);
        let view = self.2.modify(view);
        let view = self.3.modify(view);
        let view = self.4.modify(view);
        self.5.modify(view)
    }
}

impl<TView, M1, M2, M3, M4, M5, M6, M7> ModifyView<TView> for (M1, M2, M3, M4, M5, M6, M7)
where
    TView: View,
    M1: ModifyView<TView>,
    M2: ModifyView<M1::View>,
    M3: ModifyView<M2::View>,
    M4: ModifyView<M3::View>,
    M5: ModifyView<M4::View>,
    M6: ModifyView<M5::View>,
    M7: ModifyView<M6::View>,
{
    type Msg = M7::Msg;
    type View = M7::View;

    fn modify(self, view: TView) -> Self::View {
        let view = self.0.modify(view);
        let view = self.1.modify(view);
        let view = self.2.modify(view);
        let view = self.3.modify(view);
        let view = self.4.modify(view);
        let view = self.5.modify(view);
        self.6.modify(view)
    }
}

pub trait ViewExt: View {
    fn with<M>(self, modifier: M) -> M::View
    where
        Self: Sized,
        M: ModifyView<Self>,
    {
        modifier.modify(self)
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
