use crate::vdom::{Attr, CowStr, ElementContext};

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
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), Ctx::Error>
    where
        Ctx: ElementContext<Msg = TMsg>,
    {
        ctx.class(self.class_name)?;
        Ok(())
    }
}
