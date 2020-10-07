use crate::{
    attr::Attr,
    view::{element, Children, View, ViewExt as _},
};

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

macro_rules! svg_elements {
    ($( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a builder of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                attr: impl Attr<TMsg>,
                children: impl Children<TMsg>,
            ) -> impl View<Msg = TMsg> {
                element(stringify!($tag_name), Some(SVG_NAMESPACE_URI.into()))
                    .attr(attr)
                    .children(children)
            }
        }
    )*};
}

svg_elements! {
    circle,
    rect,
    line,
    polyline,
    text,
    svg,
}

/// SVG attributes.
pub mod attr {
    use crate::{
        attr::{attribute, Attribute},
        vdom::CowStr,
    };

    pub fn viewbox(min_x: i32, min_y: i32, width: i32, height: i32) -> Attribute {
        attribute(
            "viewbox",
            format!("{} {} {} {}", min_x, min_y, width, height),
        )
    }

    macro_rules! length_attributes {
        ( $( $name:ident => $attr:expr, )* ) => {$(
            pub fn $name(val: i32) -> Attribute {
                attribute($attr, val.to_string())
            }
        )*};
    }

    length_attributes! {
        width => "width",
        height => "height",
        x => "x",
        y => "y",
        x1 => "x1",
        y1 => "y1",
        x2 => "x2",
        y2 => "y2",
        cx => "cx",
        cy => "cy",
        r => "r",
        stroke_width => "stroke-width",
    }

    macro_rules! string_attributes {
        ( $( $name:ident => $attr:expr, )* ) => {$(
            pub fn $name(val: impl Into<CowStr>) -> Attribute {
                attribute($attr, val.into())
            }
        )*};
    }

    string_attributes! {
        fill => "fill",
        stroke => "stroke",
        text_anchor => "text-anchor",
        dominant_baseline => "dominant-baseline",
        stroke_linecap => "stroke-linecap",
        transform => "transform",
    }

    pub fn points(points: impl IntoIterator<Item = impl Into<(i32, i32)>>) -> Attribute {
        attribute(
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

    pub fn stroke_dasharray(value: impl IntoIterator<Item = i32>) -> Attribute {
        attribute(
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
}
