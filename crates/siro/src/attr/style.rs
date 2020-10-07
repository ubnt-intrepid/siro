use super::Attr;
use crate::{
    mailbox::Mailbox,
    vdom::{CowStr, VElement},
};

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
    fn apply<M: ?Sized>(self, element: &mut VElement, _: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element.styles.insert(self.name, self.value);
    }
}
