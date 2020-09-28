pub mod input;

mod textarea;
pub use textarea::{textarea, Textarea};

use crate::{
    builder::Element,
    vdom::{VElement, VNode},
};
use std::borrow::Cow;

pub struct HtmlElement(VElement);

impl HtmlElement {
    fn new(tag_name: Cow<'static, str>) -> Self {
        Self(VElement::new(tag_name, None))
    }
}

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

/// Create a builder of custom HTML element.
pub fn unknown(tag_name: impl Into<Cow<'static, str>>) -> HtmlElement {
    HtmlElement::new(tag_name.into())
}

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a builder of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name() -> HtmlElement {
                HtmlElement::new(stringify!($tag_name).into())
            }
        }
    )*};
}

html_elements!(
    a,          // HtmlAnchorElement
    area,       // HtmlAreaElement
    audio,      // HtmlAudioElement
    blockquote, // HtmlQuoteElement
    br,         // HtmlBRElement
    button,     // HtmlButtonElement
    canvas,     // HtmlCanvasElement
    caption,    // HtmlTableCaptionElement
    col, colgroup, // HtmlTableColElement
    data,     // HtmlDataElement
    datalist, // HtmlDataListElement
    del, ins,      // HtmlModElement
    details,  // HtmlDetailsElement
    dialog,   // HtmlDialogElement
    div,      // HtmlDivElement
    dl,       // HtmlDListElement
    embed,    // HtmlEmbedElement
    fieldset, // HtmlFieldSetElement
    form,     // HtmlFormElement
    h1, h2, h3, h4, h5, h6,       // HtmlHeadingElement
    hr,       // HtmlHRElement
    iframe,   // HtmlIFrameElement
    img,      // HtmlImageElement
    label,    // HtmlLabelElement
    legend,   // HtmlLegendElement
    li,       // HtmlLiElement
    map,      // HtmlMapElement
    meter,    // HtmlMeterElement
    object,   // HtmlObjectElement
    ol,       // HtmlOListElement
    optgroup, // HtmlOptGroupElement
    option,   // HtmlOptionElement
    output,   // HtmlOutputElement
    p,        // HtmlParagraphElement
    param,    // HtmlParamElement
    picture,  // HtmlPictureElement
    pre,      // HtmlPreElement
    progress, // HtmlProgressElement
    q,        // HtmlQuoteElement
    select,   // HtmlSelectElement
    source,   // HtmlSourceElement
    span,     // HtmlSpanElement
    table,    // HtmlTableElement
    tbody,    // HtmlTableSectionElement
    td,       // HtmlTableDataCellElement
    template, // HtmlTemplateElement
    tfoot, thead, // HtmlTableSectionElement
    th,    // HtmlTableHeaderCellElement
    time,  // HtmlTimeElement
    tr,    // HtmlTableRowElement
    track, // HtmlTrackElement
    ul,    // HtmlUListElement
    video, // HtmlVideoElement
    abbr, address, article, aside, b, bdi, bdo, cite, code, dd, dfn, dt, em, figcaption, figure,
    footer, header, i, kbd, link, main, mark, nav, noscript, rb, rp, rt, rtc, ruby, s, samp,
    section, small, sub, summary, sup, u, var,
    wbr, // HtmlElement (or HtmlSpanElement in Gecko)
);
