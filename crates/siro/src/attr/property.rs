use super::{Attr, Context};
use crate::vdom::{self, CowStr};
use wasm_bindgen::JsValue;

/// Create an `Attr` that specifies an arbitrary property value, like `domNode.name = value`.
pub fn property(name: impl Into<CowStr>, value: impl Into<vdom::Property>) -> Property {
    Property {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Property {
    name: CowStr,
    value: vdom::Property,
}

impl<TMsg: 'static> Attr<TMsg> for Property {
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.set_property(self.name, self.value)?;
        Ok(())
    }
}
