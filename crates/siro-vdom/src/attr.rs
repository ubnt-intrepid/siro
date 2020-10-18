use crate::{
    node::{ElementContext, Event, EventDecoder},
    types::{Attribute, CowStr, Property},
};
use either::Either;
use serde::Deserialize;
use std::marker::PhantomData;

/// The modifier of a `View` that annotates one or more attribute values.
pub trait Attr<TMsg: 'static> {
    /// Apply itself to specified `VElement`.
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>;
}

impl<TMsg: 'static> Attr<TMsg> for () {
    fn apply<Ctx: ?Sized>(self, _: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        Ok(())
    }
}

macro_rules! impl_attr_for_tuples {
    ( $H:ident, $( $T:ident ),* ) => {
        impl<TMsg: 'static, $H, $( $T ),*> Attr<TMsg> for ($H, $( $T ),*)
        where
            $H: Attr<TMsg>,
            $( $T: Attr<TMsg>, )*
        {
            fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
            where
                Ctx: ElementContext<Msg = TMsg>,
            {
                #[allow(non_snake_case)]
                let ($H, $( $T ),*) = self;
                $H.apply(ctx)?;
                $( $T.apply(ctx)?; )*
                Ok(())
            }
        }

        impl_attr_for_tuples!($($T),*);
    };

    ( $T:ident ) => {
        impl<TMsg: 'static, $T> Attr<TMsg> for ($T,)
        where
            $T: Attr<TMsg>,
        {
            fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
            where
                Ctx: ElementContext<Msg = TMsg>,
            {
                self.0.apply(ctx)?;
                Ok(())
            }
        }
    };
}

impl_attr_for_tuples!(
    M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, //
    M11, M12, M13, M14, M15, M16, M17, M18, M19, M20
);

impl<TMsg: 'static, T> Attr<TMsg> for Option<T>
where
    T: Attr<TMsg>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
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
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Attr::apply(l, ctx),
            Either::Right(r) => Attr::apply(r, ctx),
        }
    }
}

/// Create an `Attr` that specifies an arbitrary attribute value, like `domNode.setAttribute(name, value)`.
#[inline]
pub fn attribute<TMsg: 'static>(
    name: impl Into<CowStr>,
    value: impl Into<Attribute>,
) -> impl Attr<TMsg> {
    SetAttribute { name, value }
}

struct SetAttribute<K, V> {
    name: K,
    value: V,
}

impl<K, V, TMsg: 'static> Attr<TMsg> for SetAttribute<K, V>
where
    K: Into<CowStr>,
    V: Into<Attribute>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.attribute(self.name.into(), self.value.into())?;
        Ok(())
    }
}

/// Create an `Attr` that specifies an arbitrary property value, like `domNode.name = value`.
#[inline]
pub fn property<TMsg: 'static>(
    name: impl Into<CowStr>,
    value: impl Into<Property>,
) -> impl Attr<TMsg> {
    SetProperty { name, value }
}

struct SetProperty<K, V> {
    name: K,
    value: V,
}

impl<K, V, TMsg: 'static> Attr<TMsg> for SetProperty<K, V>
where
    K: Into<CowStr>,
    V: Into<Property>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.property(self.name.into(), self.value.into())?;
        Ok(())
    }
}

/// Create an `Attr` that specify a CSS class name.
#[inline]
pub fn class<TMsg: 'static>(class_name: impl Into<CowStr>) -> impl Attr<TMsg> {
    SetClass { class_name }
}

struct SetClass<T> {
    class_name: T,
}

impl<T, TMsg: 'static> Attr<TMsg> for SetClass<T>
where
    T: Into<CowStr>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.class(self.class_name.into())?;
        Ok(())
    }
}

/// Create an `Attr` that specify an inline style.
#[inline]
pub fn style<TMsg: 'static>(name: impl Into<CowStr>, value: impl Into<CowStr>) -> impl Attr<TMsg> {
    SetStyle { name, value }
}

struct SetStyle<K, V> {
    name: K,
    value: V,
}

impl<K, V, TMsg: 'static> Attr<TMsg> for SetStyle<K, V>
where
    K: Into<CowStr>,
    V: Into<CowStr>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.style(self.name.into(), self.value.into())?;
        Ok(())
    }
}

/// Create an `Attr` that specifies the inner HTML content of the element.
#[inline]
pub fn inner_html<TMsg: 'static>(inner_html: impl Into<CowStr>) -> impl Attr<TMsg> {
    SetInnerHtml { inner_html }
}

struct SetInnerHtml<T> {
    inner_html: T,
}

impl<T, TMsg: 'static> Attr<TMsg> for SetInnerHtml<T>
where
    T: Into<CowStr>,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.inner_html(self.inner_html.into())?;
        Ok(())
    }
}

/// Create an `Attr` that registers an event.
#[inline]
pub fn event<T, TMsg>(
    event_type: &'static str,
    f: impl Fn(T) -> Option<TMsg> + 'static,
) -> impl Attr<TMsg>
where
    T: for<'de> Deserialize<'de> + 'static,
    TMsg: 'static,
{
    OnEvent {
        event_type,
        f,
        _marker: PhantomData,
    }
}

struct OnEvent<F, T, TMsg> {
    event_type: &'static str,
    f: F,
    _marker: PhantomData<fn(T) -> TMsg>,
}

impl<F, T, TMsg> Attr<TMsg> for OnEvent<F, T, TMsg>
where
    F: Fn(T) -> Option<TMsg> + 'static,
    T: for<'de> Deserialize<'de> + 'static,
    TMsg: 'static,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.event(
            self.event_type,
            OnEventHandler {
                f: self.f,
                _marker: PhantomData,
            },
        )?;
        Ok(())
    }
}

struct OnEventHandler<F, T, TMsg> {
    f: F,
    _marker: PhantomData<fn(T) -> TMsg>,
}

impl<F, T, TMsg> EventDecoder for OnEventHandler<F, T, TMsg>
where
    F: Fn(T) -> Option<TMsg>,
    T: for<'de> Deserialize<'de>,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn decode_event<'e, E>(&self, event: E) -> Result<Option<Self::Msg>, E::Error>
    where
        E: Event<'e>,
    {
        let input = T::deserialize(event.into_deserializer())?;
        Ok((self.f)(input))
    }
}
