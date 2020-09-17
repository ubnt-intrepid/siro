use super::{element::Element, text::Text};
use std::{
    hash::{Hash, Hasher},
    rc::Weak,
};

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct NodeId(pub(crate) Weak<()>);

impl PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for NodeId {}

impl Hash for NodeId {
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
    pub(super) fn id(&self) -> NodeId {
        match self {
            Node::Element(e) => e.id(),
            Node::Text(t) => t.id(),
        }
    }
}
