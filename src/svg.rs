use crate::{
    element::Element,
    vdom::{VElement, VNode},
};
use std::{borrow::Cow, marker::PhantomData};

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

macro_rules! svg_elements {
    ($( $name:ident => $Type:ident, )*) => {$(
        mod $name {
            pub struct $Type(std::convert::Infallible);

            impl super::ElementType for $Type {
                fn name() -> &'static str {
                    stringify!($name)
                }
            }
        }

        pub type $Type = SvgElement<$name::$Type>;

        paste::paste! {
            #[doc = "Create a builder of [`<" $name ">`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/" $name ") element."]
            #[inline]
            pub fn $name() -> $Type {
                SvgElement::new()
            }
        }
    )*};
}

svg_elements! {
    circle => Circle,
    rect => Rect,
    line => Line,
    polyline => Polyline,
    text => Text,
}

pub trait ElementType {
    fn name() -> &'static str;
}

pub struct SvgElement<Type: ElementType> {
    base: VElement,
    _marker: PhantomData<Type>,
}

impl<Type: ElementType> From<SvgElement<Type>> for VNode {
    fn from(e: SvgElement<Type>) -> Self {
        e.base.into()
    }
}

impl<Type: ElementType> Element for SvgElement<Type> {
    fn as_velement(&mut self) -> &mut VElement {
        &mut self.base
    }
}

impl<Type: ElementType> SvgElement<Type> {
    fn new() -> Self {
        Self {
            base: VElement::new(Type::name().into(), Some(SVG_NAMESPACE_URI.into())),
            _marker: PhantomData,
        }
    }

    pub fn dominant_baseline(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("dominant-baseline", value.into())
    }

    pub fn fill(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("fill", value.into())
    }

    pub fn stroke(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("stroke", value.into())
    }

    pub fn stroke_dasharray(self, value: impl IntoIterator<Item = i32>) -> Self {
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

    pub fn stroke_linecap(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("stroke-linecap", value.into())
    }

    pub fn stroke_width(self, value: i32) -> Self {
        self.attribute("stroke-width", value.to_string())
    }

    pub fn text_anchor(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("text-anchor", value.into())
    }

    pub fn transform(self, transformer: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("transform", transformer.into())
    }
}

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

impl Text {
    pub fn x(self, val: i32) -> Self {
        self.attribute("x", val.to_string())
    }

    pub fn y(self, val: i32) -> Self {
        self.attribute("y", val.to_string())
    }
}

// ==== <svg> ====

/// Create a builder of [`<svg>`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg) element.
#[inline]
pub fn svg() -> Svg {
    Svg(VElement::new("svg".into(), Some(SVG_NAMESPACE_URI.into())))
}

pub struct Svg(VElement);

impl From<Svg> for VNode {
    fn from(v: Svg) -> Self {
        v.0.into()
    }
}

impl Element for Svg {
    fn as_velement(&mut self) -> &mut VElement {
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
