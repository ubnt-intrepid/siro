use super::{Attr, Context};
use crate::vdom::CowStr;
use wasm_bindgen::JsValue;

/// Create an `Attr` that specifies the inner HTML content of the element.
pub fn inner_html(inner_html: impl Into<CowStr>) -> InnerHtml {
    InnerHtml {
        inner_html: inner_html.into(),
    }
}

pub struct InnerHtml {
    inner_html: CowStr,
}

impl<TMsg: 'static> Attr<TMsg> for InnerHtml {
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = TMsg>,
    {
        ctx.set_inner_html(self.inner_html)?;
        Ok(())
    }
}
