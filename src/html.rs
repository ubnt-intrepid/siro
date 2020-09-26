pub mod input;

use crate::{
    builder::Element,
    vdom::{VElement, VNode},
};

pub struct HtmlElement(VElement);

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

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        pub fn $tag_name() -> HtmlElement {
            HtmlElement(VElement::new(stringify!($tag_name), None))
        }
    )*};
}

html_elements!(button, div, h1, h2, h3, h4, h5, h6, span);
