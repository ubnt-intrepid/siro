use crate::{
    event::{Event, EventDecoder},
    node::{Attributes, AttributesRenderer},
    types::{Attribute, CowStr, Property},
};
use serde::Deserialize;
use std::marker::PhantomData;

/// Create an `Attr` that specifies an arbitrary attribute value, like `domNode.setAttribute(name, value)`.
#[inline]
pub fn attribute<TMsg: 'static>(
    name: impl Into<CowStr>,
    value: impl Into<Attribute>,
) -> impl Attributes<TMsg> {
    SetAttribute { name, value }
}

struct SetAttribute<K, V> {
    name: K,
    value: V,
}

impl<K, V, TMsg: 'static> Attributes<TMsg> for SetAttribute<K, V>
where
    K: Into<CowStr>,
    V: Into<Attribute>,
{
    fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.attribute(self.name.into(), self.value.into())?;
        renderer.end()
    }
}

/// Create an `Attr` that specifies an arbitrary property value, like `domNode.name = value`.
#[inline]
pub fn property<TMsg: 'static>(
    name: impl Into<CowStr>,
    value: impl Into<Property>,
) -> impl Attributes<TMsg> {
    SetProperty { name, value }
}

struct SetProperty<K, V> {
    name: K,
    value: V,
}

impl<K, V, TMsg: 'static> Attributes<TMsg> for SetProperty<K, V>
where
    K: Into<CowStr>,
    V: Into<Property>,
{
    fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.property(self.name.into(), self.value.into())?;
        renderer.end()
    }
}

/// Create an `Attr` that specify a CSS class name.
#[inline]
pub fn class<TMsg: 'static>(class_name: impl Into<CowStr>) -> impl Attributes<TMsg> {
    SetClass { class_name }
}

struct SetClass<T> {
    class_name: T,
}

impl<T, TMsg: 'static> Attributes<TMsg> for SetClass<T>
where
    T: Into<CowStr>,
{
    fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.class(self.class_name.into())?;
        renderer.end()
    }
}

/// Create an `Attr` that specify an inline style.
#[inline]
pub fn style<TMsg: 'static>(
    name: impl Into<CowStr>,
    value: impl Into<CowStr>,
) -> impl Attributes<TMsg> {
    SetStyle { name, value }
}

struct SetStyle<K, V> {
    name: K,
    value: V,
}

impl<K, V, TMsg: 'static> Attributes<TMsg> for SetStyle<K, V>
where
    K: Into<CowStr>,
    V: Into<CowStr>,
{
    fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.style(self.name.into(), self.value.into())?;
        renderer.end()
    }
}

/// Create an `Attr` that specifies the inner HTML content of the element.
#[inline]
pub fn inner_html<TMsg: 'static>(inner_html: impl Into<CowStr>) -> impl Attributes<TMsg> {
    SetInnerHtml { inner_html }
}

struct SetInnerHtml<T> {
    inner_html: T,
}

impl<T, TMsg: 'static> Attributes<TMsg> for SetInnerHtml<T>
where
    T: Into<CowStr>,
{
    fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.inner_html(self.inner_html.into())?;
        renderer.end()
    }
}

/// Create an `Attr` that registers an event.
#[inline]
pub fn event<T, TMsg>(
    event_type: &'static str,
    f: impl Fn(T) -> Option<TMsg> + 'static,
) -> impl Attributes<TMsg>
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

impl<F, T, TMsg> Attributes<TMsg> for OnEvent<F, T, TMsg>
where
    F: Fn(T) -> Option<TMsg> + 'static,
    T: for<'de> Deserialize<'de> + 'static,
    TMsg: 'static,
{
    fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.event(
            self.event_type,
            OnEventDecoder {
                f: self.f,
                _marker: PhantomData,
            },
        )?;
        renderer.end()
    }
}

struct OnEventDecoder<F, T, TMsg> {
    f: F,
    _marker: PhantomData<fn(T) -> TMsg>,
}

impl<F, T, TMsg> EventDecoder for OnEventDecoder<F, T, TMsg>
where
    F: Fn(T) -> Option<TMsg>,
    T: for<'de> Deserialize<'de>,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn decode_event<E>(&self, event: E) -> Result<Option<Self::Msg>, E::Error>
    where
        E: Event,
    {
        let input = T::deserialize(event.into_deserializer())?;
        Ok((self.f)(input))
    }
}
