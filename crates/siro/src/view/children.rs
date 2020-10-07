use super::{text::text, View};
use crate::{mailbox::Mailbox, vdom::VElement};
use either::Either;

/// A trait that represents a set of child nodes.
pub trait Children<TMsg: 'static> {
    /// Append itself to `children`.
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        Self: Sized,
        M: Mailbox<Msg = TMsg>;
}

impl<TMsg: 'static> Children<TMsg> for () {
    fn append<M: ?Sized>(self, _: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
    }
}

impl<TMsg: 'static> Children<TMsg> for &'static str {
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.children.push(text(self).render(mailbox));
    }
}

impl<TMsg: 'static> Children<TMsg> for String {
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.children.push(text(self).render(mailbox));
    }
}

impl<TMsg: 'static, C> Children<TMsg> for C
where
    C: View<Msg = TMsg>,
{
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.children.push(self.render(mailbox));
    }
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl<TMsg: 'static, $H, $($T),+ > Children<TMsg> for ( $H, $($T),+ )
        where
            $H: Children<TMsg>,
            $( $T: Children<TMsg>, )+
        {
            fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
            where
                M: Mailbox<Msg = TMsg>,
            {
                #[allow(non_snake_case)]
                let ( $H, $($T),+ ) = self;

                $H.append(element, mailbox);
                $( $T.append(element, mailbox); )+
            }
        }

        impl_children_for_tuples!( $($T),+ );
    };

    ( $C:ident ) => {
        impl<TMsg: 'static, $C > Children<TMsg> for ( $C, )
        where
            $C: Children<TMsg>,
        {
            fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
            where
                M: Mailbox<Msg = TMsg>,
            {
                self.0.append(element, mailbox);
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);

/// Create a `Children` from an iterator.
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

impl<TMsg: 'static, I> Children<TMsg> for Iter<I>
where
    I: IntoIterator,
    I::Item: View<Msg = TMsg>,
{
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element
            .children
            .extend(self.iter.into_iter().map(|view| view.render(mailbox)));
    }
}

impl<TMsg: 'static, T> Children<TMsg> for Option<T>
where
    T: Children<TMsg>,
{
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        if let Some(m) = self {
            m.append(element, mailbox);
        }
    }
}

impl<TMsg: 'static, M1, M2> Children<TMsg> for Either<M1, M2>
where
    M1: Children<TMsg>,
    M2: Children<TMsg>,
{
    fn append<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => l.append(element, mailbox),
            Either::Right(r) => r.append(element, mailbox),
        }
    }
}
