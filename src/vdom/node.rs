use super::{element::Element, text::Text};
use std::{
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub(super) struct Key(Weak<()>);

impl Key {
    pub(super) fn new(rc: &Rc<()>) -> Self {
        Self(Rc::downgrade(rc))
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

pub enum Node {
    Element(Element),
    Text(Text),
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for Node {
            fn from(value: $t) -> Self {
                Self::Text(Text::from(value))
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, std::borrow::Cow<'static, str>);

impl Node {
    pub(super) fn key(&self) -> Key {
        match self {
            Node::Element(e) => e.key(),
            Node::Text(t) => t.key(),
        }
    }
}
