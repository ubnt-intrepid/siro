use super::{Context, CowStr, ElementContext, Node};
use crate::{attr::Attr, children::Children};
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
    C: Children<TMsg>,
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
    C: Children<TMsg>,
{
    type Msg = TMsg;

    fn render<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        let mut element = ctx.element_node(self.tag_name, self.namespace_uri)?;
        self.attr.apply(&mut element)?;
        self.children.diff(&mut element)?;
        element.end()
    }
}
