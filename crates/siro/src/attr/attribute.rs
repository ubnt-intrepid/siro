use super::{Attr, Context};
use crate::vdom::{self, CowStr};
use wasm_bindgen::JsValue;

/// Create an `Attr` that specifies an arbitrary attribute value, like `domNode.setAttribute(name, value)`.
pub fn attribute(name: impl Into<CowStr>, value: impl Into<vdom::Attribute>) -> Attribute {
    Attribute {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Attribute {
    name: CowStr,
    value: vdom::Attribute,
}

impl<TMsg: 'static> Attr<TMsg> for Attribute {
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.set_attribute(self.name, self.value)?;
        Ok(())
    }
}
