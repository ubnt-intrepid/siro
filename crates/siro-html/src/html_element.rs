use siro::vdom::{Element, VElement, VNode};
use std::borrow::Cow;

pub struct HtmlElement(VElement);

impl HtmlElement {
    pub(crate) fn new(tag_name: Cow<'static, str>) -> Self {
        Self(VElement::new(tag_name, None))
    }
}

impl From<HtmlElement> for VNode {
    fn from(HtmlElement(e): HtmlElement) -> Self {
        e.into()
    }
}

impl Element for HtmlElement {
    fn as_velement(&mut self) -> &mut VElement {
        &mut self.0
    }
}

/// Create a builder of custom HTML element.
pub fn unknown(tag_name: impl Into<Cow<'static, str>>) -> HtmlElement {
    HtmlElement::new(tag_name.into())
}
