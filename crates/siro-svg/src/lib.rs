/*!
SVG directives for siro.
!*/

#![doc(html_root_url = "https://docs.rs/siro-svg/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

use siro::{
    attr::Attr,
    node::{Element, ElementRenderer, Node, Nodes, NodesRenderer},
    types::{Attribute, CowStr, Property},
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
    A: Attr<TMsg>,
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

impl<TMsg: 'static, A, C> Element for SvgElement<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Nodes<TMsg>,
{
    type Msg = TMsg;

    #[inline]
    fn tag_name(&self) -> CowStr {
        self.tag_name.clone()
    }

    #[inline]
    fn namespace_uri(&self) -> Option<CowStr> {
        Some(SVG_NAMESPACE_URI.into())
    }

    fn render_element<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: ElementRenderer<Msg = Self::Msg>,
    {
        let has_inner_html = self.attr.apply(AttrContext {
            element: &mut renderer,
            has_inner_html: false,
        })?;

        if !has_inner_html {
            self.children.render_nodes(ChildrenContext {
                element: &mut renderer,
            })?;
        }

        renderer.end()
    }
}

struct AttrContext<'a, Ctx: ?Sized> {
    element: &'a mut Ctx,
    has_inner_html: bool,
}

impl<Ctx: ?Sized> siro::attr::Context for AttrContext<'_, Ctx>
where
    Ctx: ElementRenderer,
{
    type Msg = Ctx::Msg;
    type Ok = bool;
    type Error = Ctx::Error;

    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        self.element.attribute(name, value)
    }

    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        self.element.property(name, value)
    }

    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        self.element.class(class_name)
    }

    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        self.element.style(name, value)
    }

    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        self.has_inner_html = true;
        self.element.inner_html(inner_html)
    }

    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: siro::event::EventDecoder<Msg = Self::Msg> + 'static,
    {
        self.element.event(event_type, decoder)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.has_inner_html)
    }
}

struct ChildrenContext<'a, Ctx: ?Sized> {
    element: &'a mut Ctx,
}

impl<Ctx: ?Sized> NodesRenderer for ChildrenContext<'_, Ctx>
where
    Ctx: ElementRenderer,
{
    type Msg = Ctx::Msg;
    type Ok = ();
    type Error = Ctx::Error;

    #[inline]
    fn child<N>(&mut self, child: N) -> Result<(), Self::Error>
    where
        N: Node<Msg = Self::Msg>,
    {
        self.element.child(child)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

macro_rules! svg_elements {
    ($( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a `View` of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                attr: impl Attr<TMsg>,
                children: impl Nodes<TMsg>,
            ) -> impl Node<Msg = TMsg> {
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
    use siro::{
        attr::{attribute, Attr},
        types::CowStr,
    };

    macro_rules! svg_attributes {
        ( $( $name:ident => $attrname:expr, )* ) => {$(
            paste::paste! {
                #[doc = "Create an `Attr` to specify [`" $attrname "`](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/" $attrname ") attribute."]
                pub fn $name<TMsg: 'static>(val: impl Into<CowStr>) -> impl Attr<TMsg> {
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
