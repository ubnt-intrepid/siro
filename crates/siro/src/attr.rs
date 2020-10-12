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

use crate::vdom::{self, CowStr};
use either::Either;
use wasm_bindgen::JsValue;

pub trait Context {
    type Msg: 'static;

    fn set_attribute(&mut self, name: CowStr, value: vdom::Attribute) -> Result<(), JsValue>;

    fn set_property(&mut self, name: CowStr, value: vdom::Property) -> Result<(), JsValue>;

    fn set_listener<F>(&mut self, event_type: &'static str, f: F) -> Result<(), JsValue>
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static;

    fn add_class(&mut self, name: CowStr) -> Result<(), JsValue>;

    fn add_style(&mut self, name: CowStr, value: CowStr) -> Result<(), JsValue>;

    fn set_inner_html(&mut self, inner_html: CowStr) -> Result<(), JsValue>;
}

/// The modifier of a `View` that annotates one or more attribute values.
pub trait Attr<TMsg: 'static> {
    /// Apply itself to specified `VElement`.
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>;
}

impl<TMsg: 'static> Attr<TMsg> for () {
    fn apply<Ctx: ?Sized>(self, _: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        Ok(())
    }
}

macro_rules! impl_modifier_for_tuples {
    ( $H:ident, $( $T:ident ),* ) => {
        impl<TMsg: 'static, $H, $( $T ),*> Attr<TMsg> for ($H, $( $T ),*)
        where
            $H: Attr<TMsg>,
            $( $T: Attr<TMsg>, )*
        {
            fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
            where
                Ctx: Context<Msg = TMsg>,
            {
                #[allow(non_snake_case)]
                let ($H, $( $T ),*) = self;
                $H.apply(ctx)?;
                $( $T.apply(ctx)?; )*
                Ok(())
            }
        }

        impl_modifier_for_tuples!($($T),*);
    };

    ( $T:ident ) => {
        impl<TMsg: 'static, $T> Attr<TMsg> for ($T,)
        where
            $T: Attr<TMsg>,
        {
            fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
            where
                Ctx: Context<Msg = TMsg>,
            {
                self.0.apply(ctx)?;
                Ok(())
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
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        match self {
            Some(m) => m.apply(ctx),
            None => Ok(()),
        }
    }
}

impl<TMsg: 'static, M1, M2> Attr<TMsg> for Either<M1, M2>
where
    M1: Attr<TMsg>,
    M2: Attr<TMsg>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Attr::apply(l, ctx),
            Either::Right(r) => Attr::apply(r, ctx),
        }
    }
}
