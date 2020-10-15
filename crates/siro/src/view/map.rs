use super::{Context, View};
use crate::vdom::{CowStr, VElement, VNode, VText};
use gloo_events::EventListener;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

pub struct Map<TView, F> {
    pub(super) view: TView,
    pub(super) f: F,
}

impl<TView, F, TMsg> View for Map<TView, F>
where
    TView: View,
    F: Fn(TView::Msg) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        self.view.render(&mut MapContext {
            ctx,
            f: &self.f,
            _marker: PhantomData,
        })
    }

    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, old: &mut VNode) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        self.view.diff(
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
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<VElement, JsValue> {
        self.ctx.create_element(tag_name, namespace_uri)
    }

    #[inline]
    fn create_text_node(&mut self, value: CowStr) -> Result<VText, JsValue> {
        self.ctx.create_text_node(value)
    }

    fn create_listener<Callback>(
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
            .create_listener(target, event_type, move |e| callback(e).map(&f))
    }
}
