use super::{Context, CowStr, Node};
use std::marker::PhantomData;

/// Create a virtual node corresponding to an [`Text`](https://developer.mozilla.org/en-US/docs/Web/API/Text).
pub fn text<TMsg: 'static>(value: impl Into<CowStr>) -> impl Node<Msg = TMsg> {
    Text {
        value: value.into(),
        _marker: PhantomData,
    }
}

struct Text<TMsg> {
    value: CowStr,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static> Node for Text<TMsg> {
    type Msg = TMsg;

    fn render<Ctx>(self, ctx: Ctx) -> Result<Ctx::Ok, Ctx::Error>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        ctx.text_node(self.value)
    }
}
