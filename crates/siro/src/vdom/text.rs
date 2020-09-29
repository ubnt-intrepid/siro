use super::node::Key;
use std::{borrow::Cow, rc::Rc};

#[non_exhaustive]
pub struct VText {
    rc: Rc<()>,
    pub value: Cow<'static, str>,
}

impl VText {
    pub fn new<S>(value: S) -> VText
    where
        S: Into<Cow<'static, str>>,
    {
        VText {
            rc: Rc::new(()),
            value: value.into(),
        }
    }

    pub(super) fn key(&self) -> Key {
        Key::new(&self.rc)
    }
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for VText {
            fn from(value: $t) -> Self {
                Self::new(value)
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, Cow<'static, str>);
