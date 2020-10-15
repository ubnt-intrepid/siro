mod iter;

pub use iter::{iter, Iter};

use crate::view::{text, View};
use either::Either;
use wasm_bindgen::JsValue;

pub trait Context {
    type Msg: 'static;

    fn append_child<TView>(&mut self, view: TView) -> Result<(), JsValue>
    where
        TView: View<Msg = Self::Msg>;
}

/// A trait that represents a set of child nodes.
pub trait Children<TMsg: 'static> {
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>;
}

impl<TMsg: 'static> Children<TMsg> for () {
    fn diff<Ctx: ?Sized>(self, _: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        Ok(())
    }
}

impl<TMsg: 'static> Children<TMsg> for &'static str {
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        Children::diff(text(self), ctx)
    }
}

impl<TMsg: 'static> Children<TMsg> for String {
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        Children::diff(text(self), ctx)
    }
}

impl<TMsg: 'static, C> Children<TMsg> for C
where
    C: View<Msg = TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.append_child(self)?;
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
            fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
            where
                Ctx: Context<Msg = TMsg>,
            {
                let ($H, $($T),+) = self;
                Children::diff($H, ctx)?;
                $( Children::diff($T, ctx)?; )+
                Ok(())
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
            fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
            where
                Ctx: Context<Msg = TMsg>,
            {
                let ($C,) = self;
                Children::diff($C, ctx)?;
                Ok(())
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);

impl<TMsg: 'static, T> Children<TMsg> for Option<T>
where
    T: Children<TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        match self {
            Some(ch) => Children::diff(ch, ctx),
            None => Ok(()),
        }
    }
}

impl<TMsg: 'static, M1, M2> Children<TMsg> for Either<M1, M2>
where
    M1: Children<TMsg>,
    M2: Children<TMsg>,
{
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Children::diff(l, ctx),
            Either::Right(r) => Children::diff(r, ctx),
        }
    }
}
