use super::{ElementRenderer, EventDecoder, Node, Nodes, NodesRenderer, Renderer};
use crate::{
    attr::{self, Attr},
    types::{Attribute, CowStr, Property},
};
use std::marker::PhantomData;

/// Create a `Node` rendered as a DOM element.
pub fn element<TMsg: 'static, A, C>(
    tag_name: impl Into<CowStr>,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
) -> Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Nodes<TMsg>,
{
    Element {
        tag_name: tag_name.into(),
        namespace_uri,
        attr,
        children,
        _marker: PhantomData,
    }
}

pub struct Element<TMsg, A, C> {
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> Node for Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Nodes<TMsg>,
{
    type Msg = TMsg;

    fn render<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: Renderer<Msg = Self::Msg>,
    {
        let mut element = renderer.element_node(self.tag_name, self.namespace_uri)?;

        let has_inner_html = self.attr.apply(AttrContext {
            element: &mut element,
            has_inner_html: false,
        })?;

        if !has_inner_html {
            self.children.render_nodes(ChildrenContext {
                element: &mut element,
            })?;
        }

        element.end()
    }
}

struct AttrContext<'a, Ctx: ?Sized> {
    element: &'a mut Ctx,
    has_inner_html: bool,
}

impl<Ctx: ?Sized> attr::Context for AttrContext<'_, Ctx>
where
    Ctx: ElementRenderer,
{
    type Msg = Ctx::Msg;
    type Ok = bool;
    type Error = Ctx::Error;

    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        self.element.attribute(name, value)
    }

    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        self.element.property(name, value)
    }

    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        self.element.class(class_name)
    }

    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        self.element.style(name, value)
    }

    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        self.has_inner_html = true;
        self.element.inner_html(inner_html)
    }

    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        self.element.event(event_type, decoder)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.has_inner_html)
    }
}

struct ChildrenContext<'a, Ctx: ?Sized> {
    element: &'a mut Ctx,
}

impl<Ctx: ?Sized> NodesRenderer for ChildrenContext<'_, Ctx>
where
    Ctx: ElementRenderer,
{
    type Msg = Ctx::Msg;
    type Ok = ();
    type Error = Ctx::Error;

    #[inline]
    fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
    where
        N: Node<Msg = Self::Msg>,
    {
        self.element.child(child)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
