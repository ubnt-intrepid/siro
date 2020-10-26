/*!
SVG directives for siro.
!*/

#![doc(html_root_url = "https://docs.rs/siro-svg/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

use siro::{
    node::{Attributes, Nodes, NodesRenderer},
    types::CowStr,
};
use std::marker::PhantomData;

// TODO: implement missing elements and attributes.

const SVG_NAMESPACE_URI: &str = "http://www.w3.org/2000/svg";

fn svg_element<TMsg: 'static, A, C>(
    tag_name: impl Into<CowStr>,
    attr: A,
    children: C,
) -> SvgElement<TMsg, A, C>
where
    A: Attributes<TMsg>,
    C: Nodes<TMsg>,
{
    SvgElement {
        tag_name: tag_name.into(),
        attr,
        children,
        _marker: PhantomData,
    }
}

struct SvgElement<TMsg, A, C> {
    tag_name: CowStr,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> Nodes<TMsg> for SvgElement<TMsg, A, C>
where
    A: Attributes<TMsg>,
    C: Nodes<TMsg>,
{
    #[inline]
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.element(
            self.tag_name,
            Some(SVG_NAMESPACE_URI.into()),
            self.attr,
            self.children,
        )?;
        renderer.end()
    }
}

macro_rules! svg_elements {
    ($( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a `View` of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                attr: impl Attributes<TMsg>,
                children: impl Nodes<TMsg>,
            ) -> impl Nodes<TMsg> {
                svg_element(stringify!($tag_name), attr, children)
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
    use siro::{attr::attribute, node::Attributes, types::CowStr};

    macro_rules! svg_attributes {
        ( $( $name:ident => $attrname:expr, )* ) => {$(
            paste::paste! {
                #[doc = "Create an `Attr` to specify [`" $attrname "`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/" $attrname ") attribute."]
                pub fn $name<TMsg: 'static>(val: impl Into<CowStr>) -> impl Attributes<TMsg> {
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
