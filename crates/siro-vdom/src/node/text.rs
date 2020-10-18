use super::{Context, CowStr, Node};
use std::marker::PhantomData;

/// Create a `Node` rendered as a text node.
pub fn text<TMsg: 'static>(value: impl Into<CowStr>) -> Text<TMsg> {
    Text {
        value: value.into(),
        _marker: PhantomData,
    }
}

pub struct Text<TMsg> {
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
