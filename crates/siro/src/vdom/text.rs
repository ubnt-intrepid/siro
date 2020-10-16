use super::{Context, CowStr, Node};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

/// Create a virtual node corresponding to an [`Text`](https://developer.mozilla.org/en-US/docs/Web/API/Text).
pub fn text<TMsg: 'static>(value: impl Into<CowStr>) -> Text<TMsg> {
    Text {
        value: value.into(),
        _marker: PhantomData,
    }
}

/// A virtual node that will be rendered as an [`Text`](https://developer.mozilla.org/en-US/docs/Web/API/Text).
pub struct Text<TMsg> {
    value: CowStr,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static> Node for Text<TMsg> {
    type Msg = TMsg;
    type Cache = TextCache;

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<Self::Cache, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        let node = ctx.create_text_node(&*self.value)?;
        Ok(TextCache {
            node,
            value: self.value,
        })
    }

    fn diff<Ctx: ?Sized>(self, _: &mut Ctx, cache: &mut Self::Cache) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        if cache.value != self.value {
            cache.node.set_data(&*self.value);
            cache.value = self.value;
        }
        Ok(())
    }
}

pub struct TextCache {
    node: web::Text,
    value: CowStr,
}

impl AsRef<web::Node> for TextCache {
    fn as_ref(&self) -> &web::Node {
        self.node.as_ref()
    }
}
