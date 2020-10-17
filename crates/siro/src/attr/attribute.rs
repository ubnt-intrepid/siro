use crate::vdom::{self, Attr, CowStr, ElementContext};

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
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.attribute(self.name, self.value)?;
        Ok(())
    }
}
