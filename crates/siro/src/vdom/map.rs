use super::{
    types::{Attribute, CowStr, Property},
    Context, ElementContext, Node,
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

    fn render<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        self.node.render(MapContext {
            ctx,
            f: &self.f,
            _marker: PhantomData,
        })
    }
}

struct MapContext<'a, Ctx, F, TMsg> {
    ctx: Ctx,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<'a, Ctx, F, TMsg> Context for MapContext<'a, Ctx, F, TMsg>
where
    Ctx: Context,
    F: Fn(TMsg) -> Ctx::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type Ok = Ctx::Ok;
    type Error = Ctx::Error;

    type Element = MapElementContext<'a, Ctx::Element, F, TMsg>;

    fn element_node(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<Self::Element, Self::Error> {
        let element = self.ctx.element_node(tag_name, namespace_uri)?;
        Ok(MapElementContext {
            element,
            f: self.f,
            _marker: PhantomData,
        })
    }

    #[inline]
    fn text_node(self, data: CowStr) -> Result<Self::Ok, Self::Error> {
        self.ctx.text_node(data)
    }
}

pub struct MapElementContext<'a, E, F, TMsg> {
    element: E,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<E, F, TMsg> ElementContext for MapElementContext<'_, E, F, TMsg>
where
    E: ElementContext,
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
    fn event(
        &mut self,
        event_type: &'static str,
        callback: impl Fn(&web::Event) -> Option<Self::Msg> + 'static,
    ) -> Result<(), Self::Error> {
        let f = self.f.clone();
        self.element.event(event_type, move |e| callback(e).map(&f))
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
