use super::{element::VElement, text::VText};
use std::fmt;

#[non_exhaustive]
pub enum VNode {
    Element(VElement),
    Text(VText),
}

impl fmt::Debug for VNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element(e) => e.fmt(f),
            Self::Text(t) => t.fmt(f),
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

impl VNode {
    pub(crate) fn as_node(&self) -> &web::Node {
        match self {
            VNode::Element(VElement { node, .. }) => node.as_ref(),
            VNode::Text(VText { node, .. }) => node.as_ref(),
        }
    }
}
