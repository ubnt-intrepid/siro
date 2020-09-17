use super::cache::Key;
use std::{borrow::Cow, rc::Rc};

pub fn text<S>(value: S) -> Text
where
    S: Into<Cow<'static, str>>,
{
    Text {
        rc: Rc::new(()),
        value: value.into(),
    }
}

pub struct Text {
    rc: Rc<()>,
    pub(super) value: Cow<'static, str>,
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for Text {
            fn from(value: $t) -> Self {
                text(value)
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, std::borrow::Cow<'static, str>);

impl Text {
    pub(super) fn key(&self) -> Key {
        Key::new(&self.rc)
    }
}
