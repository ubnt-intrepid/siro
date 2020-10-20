use super::{CowStr, Node, Renderer};
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

    fn render<R>(self, renderer: R) -> Result<R::Ok, R::Error>
    where
        R: Renderer<Msg = Self::Msg>,
    {
        renderer.text_node(self.value)
    }
}
