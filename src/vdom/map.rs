use super::{
    AttributeValue, //
    Attributes,
    AttributesRenderer,
    CowStr,
    Event,
    EventDecoder,
    Nodes,
    NodesRenderer,
    PropertyValue,
};
use std::marker::PhantomData;

/// A virtual node created by [`map`](./trait.Node.html#method.map).
pub struct Map<TNodes, F, TMsg, UMsg> {
    nodes: TNodes,
    f: F,
    _marker: PhantomData<fn(TMsg) -> UMsg>,
}

impl<TNodes, F, TMsg, UMsg> Map<TNodes, F, TMsg, UMsg>
where
    TNodes: Nodes<TMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    pub(super) fn new(nodes: TNodes, f: F) -> Self {
        Self {
            nodes,
            f,
            _marker: PhantomData,
        }
    }
}

impl<TNodes, F, TMsg, UMsg> Nodes<UMsg> for Map<TNodes, F, TMsg, UMsg>
where
    TNodes: Nodes<TMsg>,
    F: Fn(TMsg) -> UMsg + Clone + 'static,
    TMsg: 'static,
    UMsg: 'static,
{
    fn render_nodes<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = UMsg>,
    {
        self.nodes.render_nodes(MapRenderer {
            renderer,
            f: &self.f,
            _marker: PhantomData,
        })
    }
}

struct MapRenderer<'a, R, F, TMsg, UMsg> {
    renderer: R,
    f: &'a F,
    _marker: PhantomData<fn(TMsg) -> UMsg>,
}

impl<'a, R, F, TMsg, UMsg> NodesRenderer for MapRenderer<'a, R, F, TMsg, TMsg>
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
    fn text_node(&mut self, data: CowStr) -> Result<(), Self::Error> {
        self.renderer.text_node(data)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.renderer.end()
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
    fn attribute(&mut self, name: CowStr, value: AttributeValue) -> Result<(), Self::Error> {
        self.renderer.attribute(name, value)
    }

    #[inline]
    fn property(&mut self, name: CowStr, value: PropertyValue) -> Result<(), Self::Error> {
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
        self.children.render_nodes(MapRenderer {
            renderer,
            f: self.f,
            _marker: PhantomData,
        })
    }
}
