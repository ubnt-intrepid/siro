use super::HtmlElement;
use crate::{
    element::Element,
    event::HasInputEvent,
    vdom::{VElement, VNode},
};

/// Create a builder of [`<textarea>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/textarea) element.
pub fn textarea() -> Textarea {
    Textarea::new()
}

pub struct Textarea(HtmlElement);

impl Textarea {
    fn new() -> Self {
        Self(HtmlElement::new("textarea".into()))
    }
}

impl From<Textarea> for VNode {
    fn from(t: Textarea) -> Self {
        t.0.into()
    }
}

impl Element for Textarea {
    fn as_velement(&mut self) -> &mut VElement {
        self.0.as_velement()
    }
}

impl HasInputEvent for Textarea {}
