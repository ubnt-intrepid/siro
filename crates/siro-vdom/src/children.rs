use crate::node::{IntoNode, Node};
use either::Either;
use std::marker::PhantomData;

/// A trait that represents a set of child nodes.
pub trait Children<TMsg: 'static> {
    fn render_children<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>;
}

pub trait Context {
    type Msg: 'static;
    type Ok;
    type Error;

    fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
    where
        N: Node<Msg = Self::Msg>;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<TMsg: 'static> Children<TMsg> for () {
    #[inline]
    fn render_children<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.end()
    }
}

impl<TMsg: 'static> Children<TMsg> for &'static str {
    fn render_children<Ctx>(self, mut ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.child(self.into_node())?;
        ctx.end()
    }
}

impl<TMsg: 'static> Children<TMsg> for String {
    fn render_children<Ctx>(self, mut ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.child(self.into_node())?;
        ctx.end()
    }
}

impl<TMsg: 'static, C> Children<TMsg> for C
where
    C: Node<Msg = TMsg>,
{
    fn render_children<Ctx>(self, mut ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.child(self)?;
        ctx.end()
    }
}

impl<TMsg: 'static, T> Children<TMsg> for Option<T>
where
    T: Children<TMsg>,
{
    fn render_children<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        match self {
            Some(ch) => Children::render_children(ch, ctx),
            None => ctx.end(),
        }
    }
}

impl<TMsg: 'static, M1, M2> Children<TMsg> for Either<M1, M2>
where
    M1: Children<TMsg>,
    M2: Children<TMsg>,
{
    fn render_children<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Children::render_children(l, ctx),
            Either::Right(r) => Children::render_children(r, ctx),
        }
    }
}

mod impl_tuples {
    use super::*;

    struct TupleContext<'a, Ctx: ?Sized> {
        ctx: &'a mut Ctx,
    }

    impl<Ctx: ?Sized> Context for TupleContext<'_, Ctx>
    where
        Ctx: Context,
    {
        type Msg = Ctx::Msg;
        type Ok = ();
        type Error = Ctx::Error;

        #[inline]
        fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
        where
            N: Node<Msg = Self::Msg>,
        {
            self.ctx.child(child)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl<TMsg: 'static, $H, $($T),+ > Children<TMsg> for ( $H, $($T),+ )
        where
            $H: Children<TMsg>,
            $( $T: Children<TMsg>, )+
        {
            #[allow(non_snake_case)]
            fn render_children<Ctx>(self, mut ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
            where
                Ctx: Context<Msg = TMsg>,
            {
                let ($H, $($T),+) = self;
                Children::render_children($H, TupleContext { ctx: &mut ctx })?;
                $( Children::render_children($T, TupleContext { ctx: &mut ctx })?; )+
                ctx.end()
            }
        }

        impl_children_for_tuples!( $($T),+ );
    };

    ( $C:ident ) => {
        impl<TMsg: 'static, $C > Children<TMsg> for ( $C, )
        where
            $C: Children<TMsg>,
        {
            #[allow(non_snake_case)]
            fn render_children<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
            where
                Ctx: Context<Msg = TMsg>,
            {
                Children::render_children(self.0, ctx)
            }
        }
    };
}

    impl_children_for_tuples!(
        C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
        C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
    );
}

/// Create a `Children` from an iterator of nodes.
pub fn iter<I, TMsg>(iter: I) -> impl Children<TMsg>
where
    I: IntoIterator,
    I::Item: IntoNode<TMsg>,
    TMsg: 'static,
{
    Iter {
        iter: iter.into_iter(),
        _marker: PhantomData,
    }
}

struct Iter<I, TMsg> {
    iter: I,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<I, TMsg> Children<TMsg> for Iter<I, TMsg>
where
    I: Iterator,
    I::Item: IntoNode<TMsg>,
    TMsg: 'static,
{
    fn render_children<Ctx>(self, mut ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = TMsg>,
    {
        for child in self.iter {
            ctx.child(child.into_node())?;
        }
        ctx.end()
    }
}
