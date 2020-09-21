use super::ElementBuilder;
use crate::vdom::{Element, Node};

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

pub struct SvgElement(Element);

impl From<SvgElement> for Node {
    fn from(SvgElement(e): SvgElement) -> Self {
        e.into()
    }
}

impl ElementBuilder for SvgElement {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

macro_rules! svg_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        pub fn $tag_name() -> SvgElement {
            SvgElement(Element::new(stringify!($tag_name), Some(SVG_NAMESPACE_URI)))
        }
    )*};
}

svg_elements!(circle, rect, line, polyline, text);

pub fn root() -> SvgElement {
    SvgElement(Element::new("root", Some(SVG_NAMESPACE_URI)))
}
