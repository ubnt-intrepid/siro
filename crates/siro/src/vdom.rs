//! A virtual DOM implementation in `siro`.

mod element;
mod map;
mod text;
mod types;

pub use element::{element, Attr, Children};
pub use map::Map;
pub use text::text;
pub use types::{Attribute, CowStr, Property};

/// A data structure that represents a virtual DOM node.
pub trait Node {
    /// The message type associated with this node.
    type Msg: 'static;

    /// Render this node using the given context.
    fn render<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = Self::Msg>;

    /// Map the message type of this node to another one.
    fn map<F, TMsg: 'static>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
    {
        Map { node: self, f }
    }
}

impl<T> Node for &T
where
    T: Node + Clone,
{
    type Msg = T::Msg;

    fn render<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        T::clone(self).render(ctx)
    }
}

/// The context for rendering virtual DOM nodes.
pub trait Context {
    /// The message type associated with this context.
    type Msg: 'static;

    /// The output type when the rendering completes successfully.
    type Ok;

    /// The error type on rendering.
    type Error;

    /// The context type for rendering an element node.
    ///
    /// The value of this type is returned from `element_node`.
    type Element: ElementContext<
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
pub trait ElementContext {
    type Msg: 'static;
    type Ok;
    type Error;

    /// Set an attribute to this element, corresponding to `domNode.setAttribute(name, value)`.
    fn set_attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error>;

    /// Set a property to this element, corresponding to `domNode.name = value`.
    fn set_property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error>;

    /// Set an event listener to this element.
    fn set_listener<F>(&mut self, event_type: &'static str, callback: F) -> Result<(), Self::Error>
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static;

    /// Add a class to this element.
    fn add_class(&mut self, class_name: CowStr) -> Result<(), Self::Error>;

    /// Apply an inline style to this element.
    fn add_style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error>;

    /// Set the content of inner HTML.
    ///
    /// When this method is called, the additions of child nodes by `append_child`
    /// should be ignored.
    fn set_inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error>;

    /// Append a child `Node` to this element.
    fn append_child<T>(&mut self, node: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>;

    /// Complete the rendering of this element.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}
