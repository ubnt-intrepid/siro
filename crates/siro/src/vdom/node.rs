use super::{custom::CustomNode, element::VElement, id::NodeId, text::VText, types::CowStr};
use std::fmt;

#[non_exhaustive]
pub enum VNode {
    Element(VElement),
    Text(VText),
    Custom(CustomNode),
}

impl fmt::Debug for VNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element(e) => e.fmt(f),
            Self::Text(t) => t.fmt(f),
            Self::Custom(c) => c.fmt(f),
        }
    }
}

impl From<VElement> for VNode {
    fn from(element: VElement) -> Self {
        Self::Element(element)
    }
}

impl From<VText> for VNode {
    fn from(text: VText) -> Self {
        Self::Text(text)
    }
}

impl From<CustomNode> for VNode {
    fn from(custom: CustomNode) -> Self {
        Self::Custom(custom)
    }
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for VNode {
            fn from(value: $t) -> Self {
                Self::Text(VText::from(value))
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, CowStr);

impl VNode {
    pub(crate) fn id(&self) -> NodeId {
        match self {
            VNode::Element(e) => e.id(),
            VNode::Text(t) => t.id(),
            VNode::Custom(n) => n.id(),
        }
    }
}
