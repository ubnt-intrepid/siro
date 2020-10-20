//! Representation of DOM nodes.

mod element;
mod map;
mod text;

pub use element::{element, Element};
pub use map::Map;
pub use text::{text, Text};

use crate::{
    event::EventDecoder,
    types::{Attribute, CowStr, Property},
};

/// A data structure that represents a virtual DOM node.
pub trait Node {
    /// The message type associated with this node.
    type Msg: 'static;

    /// Render this node using the given renderer.
    fn render<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: Renderer<Msg = Self::Msg>;

    /// Map the message type to another one.
    fn map<F, TMsg: 'static>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
    {
        Map { node: self, f }
    }
}

/// The context for rendering a virtual DOM node.
pub trait Renderer {
    /// The message type associated with this context.
    type Msg: 'static;

    /// The output type when the rendering completes successfully.
    type Ok;

    /// The error type on rendering.
    type Error;

    /// The renderer for an element node, returned from `element_node`.
    type Element: ElementRenderer<
        Msg = Self::Msg, //
        Ok = Self::Ok,
        Error = Self::Error,
    >;

    /// Start rendering an `Element` node.
    fn element_node(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<Self::Element, Self::Error>;

    /// Render a `Text` node.
    fn text_node(self, data: CowStr) -> Result<Self::Ok, Self::Error>;
}

/// The context for rendering an element node.
pub trait ElementRenderer {
    type Msg: 'static;
    type Ok;
    type Error;

    /// Add an attribute to this element, corresponding to `domNode.setAttribute(name, value)`.
    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error>;

    /// Add a property to this element, corresponding to `domNode.name = value`.
    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error>;

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

    /// Append a child `Node` to this element.
    fn child<T>(&mut self, node: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>;

    /// Complete the rendering of this element.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Implemented by types that is converted to `Node`.
pub trait IntoNode<TMsg: 'static> {
    /// The type of `Node` to be converted.
    type Node: Node<Msg = TMsg>;

    /// Convert itself into a `Node`.
    fn into_node(self) -> Self::Node;
}

impl<N, TMsg: 'static> IntoNode<TMsg> for N
where
    N: Node<Msg = TMsg>,
{
    type Node = Self;

    #[inline]
    fn into_node(self) -> Self::Node {
        self
    }
}

impl<TMsg: 'static> IntoNode<TMsg> for &'static str {
    type Node = Text<TMsg>;

    #[inline]
    fn into_node(self) -> Self::Node {
        text(self)
    }
}

impl<TMsg: 'static> IntoNode<TMsg> for String {
    type Node = Text<TMsg>;

    #[inline]
    fn into_node(self) -> Self::Node {
        text(self)
    }
}
