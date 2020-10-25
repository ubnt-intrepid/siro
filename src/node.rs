//! Representation of DOM nodes.

mod element;
mod iter;
mod map;
mod text;

pub use element::{element, Element};
pub use iter::iter;
pub use map::Map;
pub use text::{text, Text};

use crate::{
    event::EventDecoder,
    types::{Attribute, CowStr, Property},
};
use either::Either;

// ==== Node ====

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

// ==== Nodes ====

/// Representing a collection of virtual DOM nodes.
pub trait Nodes<TMsg: 'static> {
    fn render_nodes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>;
}

/// The rendering context specified for `Nodes`.
pub trait NodesRenderer {
    type Msg: 'static;
    type Ok;
    type Error;

    /// Append a child node.
    fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
    where
        N: Node<Msg = Self::Msg>;

    /// Finalize the rendering process.
    fn end(self) -> Result<Self::Ok, Self::Error>;
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
        renderer.child(text(self))?;
        renderer.end()
    }
}

impl<TMsg: 'static> Nodes<TMsg> for String {
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.child(text(self))?;
        renderer.end()
    }
}

impl<TMsg: 'static, C> Nodes<TMsg> for C
where
    C: Node<Msg = TMsg>,
{
    fn render_nodes<Ctx>(self, mut ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: NodesRenderer<Msg = TMsg>,
    {
        ctx.child(self)?;
        ctx.end()
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

mod impl_tuples {
    use super::*;

    struct TupleContext<'a, Ctx: ?Sized> {
        ctx: &'a mut Ctx,
    }

    impl<Ctx: ?Sized> NodesRenderer for TupleContext<'_, Ctx>
    where
        Ctx: NodesRenderer,
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
                    Nodes::render_nodes($H, TupleContext { ctx: &mut renderer })?;
                    $( Nodes::render_nodes($T, TupleContext { ctx: &mut renderer })?; )+
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
}
