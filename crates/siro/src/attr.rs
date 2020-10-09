mod attribute;
mod class;
mod inner_html;
mod property;
mod style;

pub use attribute::{attribute, Attribute};
pub use class::{class, Class};
pub use inner_html::{inner_html, InnerHtml};
pub use property::{property, Property};
pub use style::{style, Style};

use crate::{mailbox::Mailbox, vdom::VElement};
use either::Either;

/// The modifier of a `View` that annotates one or more attribute values.
pub trait Attr<TMsg: 'static> {
    /// Apply itself to specified `VElement`.
    fn apply<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>;
}

impl<TMsg: 'static> Attr<TMsg> for () {
    fn apply<M: ?Sized>(self, _: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
    }
}

macro_rules! impl_modifier_for_tuples {
    ( $H:ident, $( $T:ident ),* ) => {
        impl<TMsg: 'static, $H, $( $T ),*> Attr<TMsg> for ($H, $( $T ),*)
        where
            $H: Attr<TMsg>,
            $( $T: Attr<TMsg>, )*
        {
            fn apply<M:?Sized>(self, element: &mut VElement, mailbox: &M)
            where
                M: Mailbox<Msg = TMsg>,
            {
                #[allow(non_snake_case)]
                let ($H, $( $T ),*) = self;
                $H.apply(element, mailbox);
                $( $T.apply(element, mailbox); )*
            }
        }

        impl_modifier_for_tuples!($($T),*);
    };

    ( $T:ident ) => {
        impl<TMsg: 'static, $T> Attr<TMsg> for ($T,)
        where
            $T: Attr<TMsg>,
        {
            fn apply<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
            where
                M: Mailbox<Msg = TMsg>,
            {
                self.0.apply(element, mailbox);
            }
        }
    };
}

impl_modifier_for_tuples!(
    M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, //
    M11, M12, M13, M14, M15, M16, M17, M18, M19, M20
);

impl<TMsg: 'static, T> Attr<TMsg> for Option<T>
where
    T: Attr<TMsg>,
{
    fn apply<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let Some(m) = self {
            m.apply(element, mailbox);
        }
    }
}

impl<TMsg: 'static, M1, M2> Attr<TMsg> for Either<M1, M2>
where
    M1: Attr<TMsg>,
    M2: Attr<TMsg>,
{
    fn apply<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => l.apply(element, mailbox),
            Either::Right(r) => r.apply(element, mailbox),
        }
    }
}
