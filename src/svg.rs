use crate::{
    builder::ElementBuilder,
    vdom::{Element, Node},
};
use std::borrow::Cow;

pub mod prelude {
    pub use super::PresentationAttributes as _;
    pub use crate::builder::ElementBuilder as _;
}

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

macro_rules! svg_elements {
    ($(
        $name:ident => $T:ident,
    )*) => {$(
        pub fn $name() -> $T {
            $T(Element::new(stringify!($name), Some(SVG_NAMESPACE_URI)))
        }
    )*};
}

svg_elements! {
    svg => Svg,
    circle => Circle,
    rect => Rect,
    line => Line,
    polyline => Polyline,
    text => Text,
}

// ==== <svg> ====

pub struct Svg(Element);

impl From<Svg> for Node {
    fn from(v: Svg) -> Self {
        v.0.into()
    }
}

impl ElementBuilder for Svg {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

impl Svg {
    pub fn viewbox(self, min_x: i32, min_y: i32, width: i32, height: i32) -> Self {
        self.attribute(
            "viewbox",
            format!("{} {} {} {}", min_x, min_y, width, height),
        )
    }

    pub fn x(self, val: i32) -> Self {
        self.attribute("x", val.to_string())
    }

    pub fn y(self, val: i32) -> Self {
        self.attribute("y", val.to_string())
    }

    pub fn width(self, val: i32) -> Self {
        self.attribute("width", val.to_string())
    }

    pub fn height(self, val: i32) -> Self {
        self.attribute("height", val.to_string())
    }
}

// ==== <circle> ====

pub struct Circle(Element);

impl From<Circle> for Node {
    fn from(e: Circle) -> Self {
        e.0.into()
    }
}

impl ElementBuilder for Circle {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

impl PresentationAttributes for Circle {}

impl Circle {
    pub fn cx(self, val: i32) -> Self {
        self.attribute("cx", val.to_string())
    }

    pub fn cy(self, val: i32) -> Self {
        self.attribute("cy", val.to_string())
    }

    pub fn r(self, val: i32) -> Self {
        self.attribute("r", val.to_string())
    }
}

// ==== <rect> ====

pub struct Rect(Element);

impl From<Rect> for Node {
    fn from(e: Rect) -> Self {
        e.0.into()
    }
}

impl ElementBuilder for Rect {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

impl PresentationAttributes for Rect {}

impl Rect {
    pub fn x(self, val: i32) -> Self {
        self.attribute("x", val.to_string())
    }
    pub fn y(self, val: i32) -> Self {
        self.attribute("y", val.to_string())
    }

    pub fn width(self, val: i32) -> Self {
        self.attribute("width", val.to_string())
    }

    pub fn height(self, val: i32) -> Self {
        self.attribute("height", val.to_string())
    }

    pub fn rx(self, val: i32) -> Self {
        self.attribute("rx", val.to_string())
    }

    pub fn ry(self, val: i32) -> Self {
        self.attribute("ry", val.to_string())
    }
}

// ==== <line> ====

pub struct Line(Element);

impl From<Line> for Node {
    fn from(e: Line) -> Self {
        e.0.into()
    }
}

impl ElementBuilder for Line {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

impl PresentationAttributes for Line {}

impl Line {
    pub fn x1(self, val: i32) -> Self {
        self.attribute("x1", val.to_string())
    }

    pub fn y1(self, val: i32) -> Self {
        self.attribute("y1", val.to_string())
    }

    pub fn x2(self, val: i32) -> Self {
        self.attribute("x2", val.to_string())
    }

    pub fn y2(self, val: i32) -> Self {
        self.attribute("y2", val.to_string())
    }
}

// ==== <polyline> ====

pub struct Polyline(Element);

impl From<Polyline> for Node {
    fn from(e: Polyline) -> Self {
        e.0.into()
    }
}

impl ElementBuilder for Polyline {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

impl PresentationAttributes for Polyline {}

impl Polyline {
    pub fn points(self, points: impl IntoIterator<Item = impl Into<(i32, i32)>>) -> Self {
        self.attribute(
            "points",
            points.into_iter().fold(String::new(), |mut acc, p| {
                let (x, y) = p.into();
                if !acc.is_empty() {
                    acc += ", ";
                }
                acc += &format!("{},{}", x, y);
                acc
            }),
        )
    }
}

// ==== <text> ====

pub struct Text(Element);

impl From<Text> for Node {
    fn from(e: Text) -> Self {
        e.0.into()
    }
}

impl ElementBuilder for Text {
    fn as_element_mut(&mut self) -> &mut Element {
        &mut self.0
    }
}

impl PresentationAttributes for Text {}

impl Text {
    pub fn x(self, val: i32) -> Self {
        self.attribute("x", val.to_string())
    }

    pub fn y(self, val: i32) -> Self {
        self.attribute("y", val.to_string())
    }
}

// ==== Presentation attributes ====

pub trait PresentationAttributes: ElementBuilder {
    fn dominant_baseline(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("dominant-baseline", value.into())
    }

    fn fill(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("fill", value.into())
    }

    fn stroke(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("stroke", value.into())
    }

    fn stroke_dasharray(self, value: impl IntoIterator<Item = i32>) -> Self {
        self.attribute(
            "stroke-dasharray",
            value.into_iter().fold(String::new(), |mut acc, val| {
                if !acc.is_empty() {
                    acc += " ";
                }
                acc += &val.to_string();
                acc
            }),
        )
    }

    fn stroke_linecap(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("stroke-linecap", value.into())
    }

    fn stroke_width(self, value: i32) -> Self {
        self.attribute("stroke-width", value.to_string())
    }

    fn text_anchor(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("text-anchor", value.into())
    }

    fn transform(self, transformer: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("transform", transformer.into())
    }
}
