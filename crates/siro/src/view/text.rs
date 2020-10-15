use super::{Context, View};
use crate::vdom::{CowStr, VNode, VText};
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

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

impl<TMsg: 'static> View for Text<TMsg> {
    type Msg = TMsg;

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        Ok(ctx.create_text_node(self.value)?.into())
    }

    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, old: &mut VNode) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        match old {
            VNode::Text(VText { value, node, .. }) => {
                if *value != self.value {
                    *value = self.value;
                    node.set_data(value);
                }
            }

            _ => {
                let new = self.render(ctx)?;
                crate::util::replace_node(old.as_node(), new.as_node())?;
                *old = new;
            }
        }

        Ok(())
    }
}
