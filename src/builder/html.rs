pub mod input;

use super::ElementBuilder;
use crate::vdom::{Element, Node};

pub struct HtmlElement(Element);

impl From<HtmlElement> for Node {
    fn from(HtmlElement(e): HtmlElement) -> Self {
        e.into()
    }
}

impl ElementBuilder for HtmlElement {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        pub fn $tag_name() -> HtmlElement {
            HtmlElement(Element::new(stringify!($tag_name), None))
        }
    )*};
}

html_elements!(button, div, h1, h2, h3, h4, h5, h6, span);
