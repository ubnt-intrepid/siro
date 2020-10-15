use super::{Attr, Context};
use crate::vdom::CowStr;
use wasm_bindgen::JsValue;

/// Create an `Attr` that specify an inline style.
pub fn style(name: impl Into<CowStr>, value: impl Into<CowStr>) -> Style {
    Style {
        name: name.into(),
        value: value.into(),
    }
}

pub struct Style {
    name: CowStr,
    value: CowStr,
}

impl<TMsg: 'static> Attr<TMsg> for Style {
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.add_style(self.name, self.value)?;
        Ok(())
    }
}
