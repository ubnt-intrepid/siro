use crate::view::{element, ModifyView, View, ViewExt as _};

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a builder of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                m: impl ModifyView<TMsg>,
            ) -> impl View<Msg = TMsg> {
                element(stringify!($tag_name), None).with(m)
            }
        }
    )*};
}

// HtmlElement (or HtmlSpanElement in Gecko)
html_elements!(
    abbr, address, article, aside, b, bdi, bdo, cite, code, dd, dfn, dt, em, figcaption, figure,
    footer, header, i, kbd, link, main, mark, nav, noscript, rb, rp, rt, rtc, ruby, s, samp,
    section, small, strong, sub, summary, sup, u, var, wbr,
);

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
    input,    // HtmlInputElement
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
    textarea, // HtmlTextAreaElement
    tfoot, thead, // HtmlTableSectionElement
    th,    // HtmlTableHeaderCellElement
    time,  // HtmlTimeElement
    tr,    // HtmlTableRowElement
    track, // HtmlTrackElement
    ul,    // HtmlUListElement
    video, // HtmlVideoElement
);
