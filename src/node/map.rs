use super::{Element, ElementRenderer, Node, NodeRenderer};
use crate::{
    event::{Event, EventDecoder},
    types::{Attribute, CowStr, Property},
};
use std::marker::PhantomData;

/// A virtual node created by [`map`](./trait.Node.html#method.map).
pub struct Map<TNode, F> {
    pub(super) node: TNode,
    pub(super) f: F,
}

impl<TNode, F, TMsg> Node for Map<TNode, F>
where
    TNode: Node,
    F: Fn(TNode::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn render<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodeRenderer<Msg = Self::Msg>,
    {
        self.node.render(MapRenderer {
            renderer,
            f: &self.f,
            _marker: PhantomData,
        })
    }
}

struct MapRenderer<'a, R, F, TMsg> {
    renderer: R,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<'a, R, F, TMsg> NodeRenderer for MapRenderer<'a, R, F, TMsg>
where
    R: NodeRenderer,
    F: Fn(TMsg) -> R::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Ok = R::Ok;
    type Error = R::Error;

    #[inline]
    fn element<E>(self, element: E) -> Result<Self::Ok, Self::Error>
    where
        E: Element<Msg = Self::Msg>,
    {
        self.renderer.element(MapElement { element, f: self.f })
    }

    #[inline]
    fn text(self, data: CowStr) -> Result<Self::Ok, Self::Error> {
        self.renderer.text(data)
    }
}

struct MapElement<'a, E, F> {
    element: E,
    f: &'a F,
}

impl<E, F, TMsg> Element for MapElement<'_, E, F>
where
    E: Element,
    F: Fn(E::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    #[inline]
    fn tag_name(&self) -> CowStr {
        self.element.tag_name()
    }

    #[inline]
    fn namespace_uri(&self) -> Option<CowStr> {
        self.element.namespace_uri()
    }

    #[inline]
    fn render_element<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: ElementRenderer<Msg = Self::Msg>,
    {
        self.element.render_element(MapElementRenderer {
            element: renderer,
            f: self.f,
            _marker: PhantomData,
        })
    }
}

struct MapElementRenderer<'a, E, F, TMsg> {
    element: E,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<E, F, TMsg> ElementRenderer for MapElementRenderer<'_, E, F, TMsg>
where
    E: ElementRenderer,
    F: Fn(TMsg) -> E::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Ok = E::Ok;
    type Error = E::Error;

    #[inline]
    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        self.element.attribute(name, value)
    }

    #[inline]
    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        self.element.property(name, value)
    }

    #[inline]
    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        self.element.event(
            event_type,
            MapEventDecoder {
                decoder,
                f: self.f.clone(),
            },
        )
    }

    #[inline]
    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        self.element.class(class_name)
    }

    #[inline]
    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        self.element.style(name, value)
    }

    #[inline]
    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        self.element.inner_html(inner_html)
    }

    #[inline]
    fn child<T>(&mut self, node: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>,
    {
        self.element.child(node.map(self.f.clone()))
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.element.end()
    }
}

struct MapEventDecoder<D, F> {
    decoder: D,
    f: F,
}

impl<D, F, TMsg: 'static> EventDecoder for MapEventDecoder<D, F>
where
    D: EventDecoder,
    F: Fn(D::Msg) -> TMsg,
{
    type Msg = TMsg;

    fn decode_event<E>(&self, event: E) -> Result<Option<Self::Msg>, E::Error>
    where
        E: Event,
    {
        Ok(self.decoder.decode_event(event)?.map(&self.f))
    }
}
