use super::{Attributes, AttributesRenderer, Node, NodeRenderer, Nodes, NodesRenderer};
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
    fn element<A, C>(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attr: A,
        children: C,
    ) -> Result<Self::Ok, Self::Error>
    where
        A: Attributes<Self::Msg>,
        C: Nodes<Self::Msg>,
    {
        self.renderer.element(
            tag_name,
            namespace_uri,
            MapAttributes {
                attr,
                f: self.f,
                _marker: PhantomData,
            },
            MapChildren {
                children,
                f: self.f,
                _marker: PhantomData,
            },
        )
    }

    #[inline]
    fn text(self, data: CowStr) -> Result<Self::Ok, Self::Error> {
        self.renderer.text(data)
    }
}

struct MapAttributes<'a, A, F, TMsg> {
    attr: A,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<A, F, TMsg, UMsg> Attributes<UMsg> for MapAttributes<'_, A, F, TMsg>
where
    A: Attributes<TMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    fn render_attributes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: AttributesRenderer<Msg = UMsg>,
    {
        self.attr.render_attributes(MapAttributesRenderer {
            renderer,
            f: self.f,
            _marker: PhantomData,
        })
    }
}

struct MapAttributesRenderer<'a, R, F, TMsg, UMsg> {
    renderer: R,
    f: &'a F,
    _marker: PhantomData<fn(TMsg) -> UMsg>,
}

impl<R, F, TMsg, UMsg> AttributesRenderer for MapAttributesRenderer<'_, R, F, TMsg, UMsg>
where
    R: AttributesRenderer<Msg = UMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    type Msg = TMsg;
    type Ok = R::Ok;
    type Error = R::Error;

    #[inline]
    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        self.renderer.attribute(name, value)
    }

    #[inline]
    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        self.renderer.property(name, value)
    }

    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        self.renderer.event(
            event_type,
            MapEventDecoder {
                decoder,
                f: self.f.clone(),
            },
        )
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
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.renderer.end()
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

struct MapChildren<'a, C, F, TMsg> {
    children: C,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<C, F, TMsg, UMsg> Nodes<UMsg> for MapChildren<'_, C, F, TMsg>
where
    C: Nodes<TMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    fn render_nodes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = UMsg>,
    {
        self.children.render_nodes(MapChildrenRenderer {
            renderer,
            f: self.f,
            _marker: PhantomData,
        })
    }
}

struct MapChildrenRenderer<'a, R, F, TMsg, UMsg> {
    renderer: R,
    f: &'a F,
    _marker: PhantomData<fn(TMsg) -> UMsg>,
}

impl<R, F, TMsg, UMsg> NodesRenderer for MapChildrenRenderer<'_, R, F, TMsg, UMsg>
where
    R: NodesRenderer<Msg = UMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    type Msg = TMsg;
    type Ok = R::Ok;
    type Error = R::Error;

    #[inline]
    fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
    where
        N: Node<Msg = Self::Msg>,
    {
        self.renderer.child(child.map(self.f.clone()))
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.renderer.end()
    }
}
