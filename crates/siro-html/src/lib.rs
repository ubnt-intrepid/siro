//! HTML directives.

use siro::{
    attr::Attr,
    view::{element, Children, View, ViewExt as _},
};

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a `View` of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                attr: impl Attr<TMsg>,
                children: impl Children<TMsg>
            ) -> impl View<Msg = TMsg> {
                element(stringify!($tag_name), None)
                    .attr(attr)
                    .children(children)
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

/// HTML attributes.
pub mod attr {
    use siro::{
        attr::{attribute, property, Attribute, Property},
        vdom,
    };

    pub fn autofocus(autofocus: bool) -> Attribute {
        attribute("autofocus", autofocus)
    }

    pub fn href(url: impl Into<vdom::CowStr>) -> Attribute {
        attribute("href", url.into())
    }

    pub fn id(id: impl Into<vdom::CowStr>) -> Attribute {
        attribute("id", id.into())
    }

    pub fn label_for(target_id: impl Into<vdom::CowStr>) -> Attribute {
        attribute("for", target_id.into())
    }

    pub fn name(name: impl Into<vdom::CowStr>) -> Attribute {
        attribute("name", name.into())
    }

    pub fn placeholder(placeholder: impl Into<vdom::CowStr>) -> Attribute {
        attribute("placeholder", placeholder.into())
    }

    pub fn checked(checked: bool) -> Property {
        property("checked", checked)
    }

    pub fn value(value: impl Into<vdom::Property>) -> Property {
        property("value", value)
    }
}

/// `View`s for [`<input>`] with specific element type.
///
/// [`<input>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input
pub mod input {
    use siro::{
        attr::{attribute, Attr},
        view::{Children, View},
    };

    macro_rules! input_elements {
        ( $( $type_name:ident ),* $(,)? ) => {$(
            paste::paste! {
                #[doc = "Create a `View` of [`<input type=\"" $type_name "\">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/" $type_name ") element."]
                #[inline]
                pub fn $type_name<TMsg: 'static>(
                    attr: impl Attr<TMsg>,
                    children: impl Children<TMsg>
                ) -> impl View<Msg = TMsg> {
                    super::input((attribute("type", stringify!($type_name)), attr), children)
                }
            }
        )*};
    }

    input_elements!(
        button, checkbox, color, date, email, file, hidden, image, month, number, password, radio,
        range, reset, search, submit, tel, text, time, url, week,
    );

    /// Create a `View` of [`<input type="datetime-local">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/datetime-local) element.
    #[inline]
    pub fn datetime_local<TMsg: 'static>(
        attr: impl Attr<TMsg>,
        children: impl Children<TMsg>,
    ) -> impl View<Msg = TMsg> {
        super::input((attribute("type", "datetime-local"), attr), children)
    }
}
