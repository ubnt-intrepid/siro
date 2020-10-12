use super::{Attr, Context};
use crate::vdom::CowStr;
use wasm_bindgen::JsValue;

/// Create an `Attr` that specify a CSS class name.
pub fn class(class_name: impl Into<CowStr>) -> Class {
    Class {
        class_name: class_name.into(),
    }
}

pub struct Class {
    class_name: CowStr,
}

impl<TMsg: 'static> Attr<TMsg> for Class {
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.add_class(self.class_name)?;
        Ok(())
    }
}
