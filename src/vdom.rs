//! Representation of DOM nodes.

mod map;

pub use map::Map;

use either::Either;
use serde::de::{self, Deserialize, Deserializer};
use std::marker::PhantomData;

/// Clone-on-write string.
pub type CowStr = std::borrow::Cow<'static, str>;

// ==== Nodes ====

/// A collection of virtual DOM nodes.
pub trait Nodes<TMsg: 'static> {
    /// Render nodes using the given renderer.
    fn render_nodes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>;

    /// Map the message values to another type.
    #[inline]
    fn map<F, UMsg: 'static>(self, f: F) -> Map<Self, F, TMsg, UMsg>
    where
        Self: Sized,
        F: Fn(TMsg) -> UMsg + Clone + 'static,
    {
        Map::new(self, f)
    }
}

/// The context for rendering a `Nodes`.
pub trait NodesRenderer {
    type Msg: 'static;
    type Ok;
    type Error;

    /// Render a virtual [`Element`].
    ///
    /// [`Element`]: https://developer.mozilla.org/en-US/docs/Web/API/Element
    fn element<A, C>(
        &mut self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attr: A,
        children: C,
    ) -> Result<(), Self::Error>
    where
        A: Attributes<Self::Msg>,
        C: Nodes<Self::Msg>;

    /// Render a virtual [`Text`] node.
    ///
    /// [`Text`]: https://developer.mozilla.org/en-US/docs/Web/API/Text
    fn text_node(&mut self, data: CowStr) -> Result<(), Self::Error>;

    /// Finalize the rendering process.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<R: ?Sized> NodesRenderer for &mut R
where
    R: NodesRenderer,
{
    type Msg = R::Msg;
    type Ok = ();
    type Error = R::Error;

    #[inline]
    fn element<A, C>(
        &mut self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attr: A,
        children: C,
    ) -> Result<(), Self::Error>
    where
        A: Attributes<Self::Msg>,
        C: Nodes<Self::Msg>,
    {
        (*self).element(tag_name, namespace_uri, attr, children)
    }

    #[inline]
    fn text_node(&mut self, data: CowStr) -> Result<(), Self::Error> {
        (*self).text_node(data)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<TMsg: 'static> Nodes<TMsg> for () {
    #[inline]
    fn render_nodes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.end()
    }
}

impl<TMsg: 'static> Nodes<TMsg> for &'static str {
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.text_node(self.into())?;
        renderer.end()
    }
}

impl<TMsg: 'static> Nodes<TMsg> for String {
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.text_node(self.into())?;
        renderer.end()
    }
}

impl<TMsg: 'static, T> Nodes<TMsg> for Option<T>
where
    T: Nodes<TMsg>,
{
    fn render_nodes<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: NodesRenderer<Msg = TMsg>,
    {
        match self {
            Some(ch) => Nodes::render_nodes(ch, ctx),
            None => ctx.end(),
        }
    }
}

impl<TMsg: 'static, M1, M2> Nodes<TMsg> for Either<M1, M2>
where
    M1: Nodes<TMsg>,
    M2: Nodes<TMsg>,
{
    fn render_nodes<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: NodesRenderer<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Nodes::render_nodes(l, ctx),
            Either::Right(r) => Nodes::render_nodes(r, ctx),
        }
    }
}

macro_rules! impl_nodes_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl<TMsg: 'static, $H, $($T),+ > Nodes<TMsg> for ( $H, $($T),+ )
        where
            $H: Nodes<TMsg>,
            $( $T: Nodes<TMsg>, )+
        {
            #[allow(non_snake_case)]
            fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
            where
                R: NodesRenderer<Msg = TMsg>,
            {
                let ($H, $($T),+) = self;
                Nodes::render_nodes($H, &mut renderer)?;
                $( Nodes::render_nodes($T, &mut renderer)?; )+
                renderer.end()
            }
        }

        impl_nodes_for_tuples!( $($T),+ );
    };

    ( $C:ident ) => {
        impl<TMsg: 'static, $C > Nodes<TMsg> for ( $C, )
        where
            $C: Nodes<TMsg>,
        {
            #[allow(non_snake_case)]
            fn render_nodes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
            where
                R: NodesRenderer<Msg = TMsg>,
            {
                Nodes::render_nodes(self.0, renderer)
            }
        }
    };
}

impl_nodes_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);

/// Create a `Nodes` from an iterator.
pub fn iter<I, TMsg>(iter: I) -> impl Nodes<TMsg>
where
    I: IntoIterator,
    I::Item: Nodes<TMsg>,
    TMsg: 'static,
{
    IterNodes {
        iter: iter.into_iter(),
        _marker: PhantomData,
    }
}

struct IterNodes<I, TMsg> {
    iter: I,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<I, TMsg> Nodes<TMsg> for IterNodes<I, TMsg>
where
    I: Iterator,
    I::Item: Nodes<TMsg>,
    TMsg: 'static,
{
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        for child in self.iter {
            Nodes::render_nodes(child, &mut renderer)?;
        }
        renderer.end()
    }
}

// ==== Attributes ====

/// A collection of DOM attributes.
pub trait Attributes<TMsg: 'static> {
    /// Apply DOM attributes to specified context.
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>;
}

/// A rendering context restricted to DOM attributes.
pub trait AttributesRenderer {
    type Msg: 'static;
    type Ok;
    type Error;

    /// Add an attribute to this element, corresponding to `domNode.setAttribute(name, value)`.
    fn attribute(&mut self, name: CowStr, value: AttributeValue) -> Result<(), Self::Error>;

    /// Add a property to this element, corresponding to `domNode.name = value`.
    fn property(&mut self, name: CowStr, value: PropertyValue) -> Result<(), Self::Error>;

    /// Register an event callback to this element.
    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static;

    /// Add a class to this element.
    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error>;

    /// Apply an inline style to this element.
    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error>;

    /// Set the content of inner HTML.
    ///
    /// When this method is called, the additions of child nodes by `append_child`
    /// should be ignored.
    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error>;

    /// Complete the rendering of this element.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<R: ?Sized> AttributesRenderer for &mut R
where
    R: AttributesRenderer,
{
    type Msg = R::Msg;
    type Ok = ();
    type Error = R::Error;

    #[inline]
    fn attribute(&mut self, name: CowStr, value: AttributeValue) -> Result<(), Self::Error> {
        (*self).attribute(name, value)
    }

    #[inline]
    fn property(&mut self, name: CowStr, value: PropertyValue) -> Result<(), Self::Error> {
        (*self).property(name, value)
    }

    #[inline]
    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        (*self).event(event_type, decoder)
    }

    #[inline]
    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        (*self).class(class_name)
    }

    #[inline]
    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        (*self).style(name, value)
    }

    #[inline]
    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        (*self).inner_html(inner_html)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<TMsg: 'static> Attributes<TMsg> for () {
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.end()
    }
}

macro_rules! impl_attr_for_tuples {
        ( $H:ident, $( $T:ident ),* ) => {
            impl_attr_for_tuples!($($T),*);

            impl<TMsg: 'static, $H, $( $T ),*> Attributes<TMsg> for ($H, $( $T ),*)
            where
                $H: Attributes<TMsg>,
                $( $T: Attributes<TMsg>, )*
            {
                fn render_attributes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
                where
                    R: AttributesRenderer<Msg = TMsg>,
                {
                    #[allow(non_snake_case)]
                    let ($H, $( $T ),*) = self;
                $H.render_attributes(&mut renderer)?;
                $( $T.render_attributes(&mut renderer)?; )*
                    renderer.end()
                }
            }
        };

        ( $T:ident ) => {
            impl<TMsg: 'static, $T> Attributes<TMsg> for ($T,)
            where
                $T: Attributes<TMsg>,
            {
            #[inline]
                fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
                where
                    R: AttributesRenderer<Msg = TMsg>,
                {
                    self.0.render_attributes(renderer)
                }
            }
        };
    }

impl_attr_for_tuples!(
    M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, //
    M11, M12, M13, M14, M15, M16, M17, M18, M19, M20
);

impl<TMsg: 'static, T> Attributes<TMsg> for Option<T>
where
    T: Attributes<TMsg>,
{
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        match self {
            Some(attrs) => attrs.render_attributes(renderer),
            None => renderer.end(),
        }
    }
}

impl<TMsg: 'static, M1, M2> Attributes<TMsg> for Either<M1, M2>
where
    M1: Attributes<TMsg>,
    M2: Attributes<TMsg>,
{
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        match self {
            Either::Left(l) => Attributes::render_attributes(l, renderer),
            Either::Right(r) => Attributes::render_attributes(r, renderer),
        }
    }
}

/// The value of DOM attributes.
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    String(CowStr),
    Bool(bool),
}

macro_rules! impl_attributes {
    ($(
        $Variant:ident => [ $($t:ty),* $(,)? ];
    )*) => {$(
        $(
            impl From<$t> for AttributeValue {
                fn from(val: $t) -> Self {
                    Self::$Variant(val.into())
                }
            }
        )*
    )*};
}

impl_attributes! {
    String => [
        &'static str,
        String,
        CowStr,
    ];
    Bool => [bool];
}

/// The property values in DOM object.
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    String(CowStr),
    Number(f64),
    Bool(bool),
}

macro_rules! impl_properties {
    ($(
        $Variant:ident => [ $($t:ty),* $(,)? ];
    )*) => {$(
        $(
            impl From<$t> for PropertyValue {
                fn from(val: $t) -> Self {
                    Self::$Variant(val.into())
                }
            }
        )*
    )*};
}

impl_properties! {
    String => [
        &'static str,
        String,
        CowStr,
    ];
    Number => [
        f64, f32,
        i8, i16, i32,
        u8, u16, u32,
    ];
    Bool => [bool];
}

/// An abstraction of DOM events.
pub trait Event {
    /// The type of deserializer returned from `into_deserializer`.
    type Deserializer: for<'de> Deserializer<'de, Error = Self::Error>;
    /// The error type of deserializer.
    type Error: de::Error;

    /// Convert itself into a `Deserializer`.
    fn into_deserializer(self) -> Self::Deserializer;

    /// Deserialize the event value to specified type.
    fn decode<T>(self) -> Result<T, Self::Error>
    where
        Self: Sized,
        T: for<'de> Deserialize<'de>,
    {
        T::deserialize(self.into_deserializer())
    }
}

/// Decoder of DOM events.
pub trait EventDecoder {
    /// The message type decoded from events.
    type Msg: 'static;

    /// Decode an `Event` to specific message type.
    fn decode_event<E>(&self, event: E) -> Result<Option<Self::Msg>, E::Error>
    where
        E: Event;
}

/// Create an `Attr` that specifies an arbitrary attribute value, like `domNode.setAttribute(name, value)`.
#[inline]
pub fn attribute<TMsg: 'static>(
    name: impl Into<CowStr>,
    value: impl Into<AttributeValue>,
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
    V: Into<AttributeValue>,
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
    value: impl Into<PropertyValue>,
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
    V: Into<PropertyValue>,
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
