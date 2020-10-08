//! SVG directives.

use siro::{
    attr::Attr,
    view::{element, Children, View, ViewExt as _},
};

// TODO: implement missing elements and attributes.

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

macro_rules! svg_elements {
    ($( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a `View` of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/" $tag_name ") element."]
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
    use siro::{
        attr::{attribute, Attribute},
        vdom::CowStr,
    };

    macro_rules! svg_attributes {
        ( $( $name:ident => $attrname:expr, )* ) => {$(
            paste::paste! {
                #[doc = "Create an `Attr` to specify [`" $attrname "`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/" $attrname ") attribute."]
                pub fn $name(val: impl Into<CowStr>) -> Attribute {
                    attribute($attrname, val.into())
                }
            }
        )*};
    }

    svg_attributes! {
        cx => "cx",
        cy => "cy",
        dominant_baseline => "dominant-baseline",
        fill => "fill",
        height => "height",
        points => "points",
        r => "r",
        stroke => "stroke",
        stroke_dasharray => "stroke-dasharray",
        stroke_linecap => "stroke-linecap",
        stroke_width => "stroke-width",
        text_anchor => "text-anchor",
        transform => "transform",
        viewbox => "viewbox",
        width => "width",
        x => "x",
        x1 => "x1",
        x2 => "x2",
        y => "y",
        y1 => "y1",
        y2 => "y2",
    }
}
