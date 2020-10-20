use super::{ElementRenderer, Node, Renderer};
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
        R: Renderer<Msg = Self::Msg>,
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

impl<'a, R, F, TMsg> Renderer for MapRenderer<'a, R, F, TMsg>
where
    R: Renderer,
    F: Fn(TMsg) -> R::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Ok = R::Ok;
    type Error = R::Error;

    type Element = MapElementRenderer<'a, R::Element, F, TMsg>;

    fn element_node(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<Self::Element, Self::Error> {
        let element = self.renderer.element_node(tag_name, namespace_uri)?;
        Ok(MapElementRenderer {
            element,
            f: self.f,
            _marker: PhantomData,
        })
    }

    #[inline]
    fn text_node(self, data: CowStr) -> Result<Self::Ok, Self::Error> {
        self.renderer.text_node(data)
    }
}

pub struct MapElementRenderer<'a, E, F, TMsg> {
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
