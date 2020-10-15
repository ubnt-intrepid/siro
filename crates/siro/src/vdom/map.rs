use super::{Context, Node, VNode};
use gloo_events::EventListener;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

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

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        self.node.render(&mut MapContext {
            ctx,
            f: &self.f,
            _marker: PhantomData,
        })
    }

    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, old: &mut VNode) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        self.node.diff(
            &mut MapContext {
                ctx,
                f: &self.f,
                _marker: PhantomData,
            },
            old,
        )
    }
}

struct MapContext<'a, Ctx: ?Sized, F, TMsg: 'static> {
    ctx: &'a mut Ctx,
    f: &'a F,
    _marker: PhantomData<fn(TMsg)>,
}

impl<Ctx: ?Sized, F, TMsg> Context for MapContext<'_, Ctx, F, TMsg>
where
    Ctx: Context,
    F: Fn(TMsg) -> Ctx::Msg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    #[inline]
    fn create_element(
        &mut self,
        tag_name: &str,
        namespace_uri: Option<&str>,
    ) -> Result<web::Element, JsValue> {
        self.ctx.create_element(tag_name, namespace_uri)
    }

    #[inline]
    fn create_text_node(&mut self, value: &str) -> Result<web::Text, JsValue> {
        self.ctx.create_text_node(value)
    }

    fn set_listener<Callback>(
        &mut self,
        target: &web::EventTarget,
        event_type: &'static str,
        callback: Callback,
    ) -> EventListener
    where
        Callback: Fn(&web::Event) -> Option<Self::Msg> + 'static,
    {
        let f = self.f.clone();
        self.ctx
            .set_listener(target, event_type, move |e| callback(e).map(&f))
    }
}
