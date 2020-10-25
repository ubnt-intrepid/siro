//! Representation of DOM nodes.

mod iter;
mod map;
mod text;

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
        R: NodeRenderer<Msg = Self::Msg>;

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
pub trait NodeRenderer {
    /// The message type associated with this context.
    type Msg: 'static;

    /// The output type when the rendering completes successfully.
    type Ok;

    /// The error type on rendering.
    type Error;

    /// Render an `Element` node.
    fn element<A, C>(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attr: A,
        children: C,
    ) -> Result<Self::Ok, Self::Error>
    where
        A: Attributes<Self::Msg>,
        C: Nodes<Self::Msg>;

    /// Render a `Text` node.
    fn text(self, data: CowStr) -> Result<Self::Ok, Self::Error>;
}

/// A collection of DOM attributes.
pub trait Attributes<TMsg: 'static> {
    /// Apply DOM attributes to specified context.
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>;
}

pub trait AttributesRenderer {
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

    /// Complete the rendering of this element.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

impl<TMsg: 'static> Attributes<TMsg> for () {
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = TMsg>,
    {
        renderer.end()
    }
}

mod impl_attr_for_tuples {
    use super::*;

    struct RenderTupleAttributes<'a, R: ?Sized> {
        renderer: &'a mut R,
    }

    impl<R: ?Sized> AttributesRenderer for RenderTupleAttributes<'_, R>
    where
        R: AttributesRenderer,
    {
        type Msg = R::Msg;
        type Ok = ();
        type Error = R::Error;

        #[inline]
        fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
            self.renderer.attribute(name, value)
        }

        #[inline]
        fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
            self.renderer.property(name, value)
        }

        #[inline]
        fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
            self.renderer.class(class_name)
        }

        #[inline]
        fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
            self.renderer.style(name, value)
        }

        #[inline]
        fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
            self.renderer.inner_html(inner_html)
        }

        #[inline]
        fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
        where
            D: EventDecoder<Msg = Self::Msg> + 'static,
        {
            self.renderer.event(event_type, decoder)
        }

        #[inline]
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
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

                    $H.render_attributes(RenderTupleAttributes {
                        renderer: &mut renderer,
                    })?;

                    $(
                        $T.render_attributes(RenderTupleAttributes {
                            renderer: &mut renderer,
                        })?;
                    )*

                    renderer.end()
                }
            }
        };

        ( $T:ident ) => {
            impl<TMsg: 'static, $T> Attributes<TMsg> for ($T,)
            where
                $T: Attributes<TMsg>,
            {
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
}

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
