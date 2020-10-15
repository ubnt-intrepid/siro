use super::{Context, CowStr, Node, VNode};
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

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        let node = ctx.create_text_node(&*self.value)?;
        Ok(VNode::Text(VText {
            node,
            value: self.value,
        }))
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
                crate::util::replace_node(old.as_ref(), new.as_ref())?;
                *old = new;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct VText {
    node: web::Text,
    value: CowStr,
}

impl AsRef<web::Node> for VText {
    fn as_ref(&self) -> &web::Node {
        self.node.as_ref()
    }
}
