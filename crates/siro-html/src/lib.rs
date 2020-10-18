//! HTML directives.

use siro_vdom::{
    attr::Attr,
    children::Children,
    node::{element, Node},
};

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a `View` of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                attr: impl Attr<TMsg>,
                children: impl Children<TMsg>
            ) -> impl Node<Msg = TMsg> {
                element(stringify!($tag_name), None, attr, children)
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
    use siro_vdom::{
        attr::{attribute, property, Attr},
        types::{CowStr, Property},
    };

    pub fn autofocus<TMsg: 'static>(autofocus: bool) -> impl Attr<TMsg> {
        attribute("autofocus", autofocus)
    }

    pub fn href<TMsg: 'static>(url: impl Into<CowStr>) -> impl Attr<TMsg> {
        attribute("href", url.into())
    }

    pub fn id<TMsg: 'static>(id: impl Into<CowStr>) -> impl Attr<TMsg> {
        attribute("id", id.into())
    }

    pub fn label_for<TMsg: 'static>(target_id: impl Into<CowStr>) -> impl Attr<TMsg> {
        attribute("for", target_id.into())
    }

    pub fn name<TMsg: 'static>(name: impl Into<CowStr>) -> impl Attr<TMsg> {
        attribute("name", name.into())
    }

    pub fn placeholder<TMsg: 'static>(placeholder: impl Into<CowStr>) -> impl Attr<TMsg> {
        attribute("placeholder", placeholder.into())
    }

    pub fn checked<TMsg: 'static>(checked: bool) -> impl Attr<TMsg> {
        property("checked", checked)
    }

    pub fn value<TMsg: 'static>(value: impl Into<Property>) -> impl Attr<TMsg> {
        property("value", value)
    }
}

/// `View`s for [`<input>`] with specific element type.
///
/// [`<input>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input
pub mod input {
    use siro_vdom::{
        attr::{attribute, Attr},
        node::Node,
    };

    macro_rules! input_elements {
        ( $( $type_name:ident ),* $(,)? ) => {$(
            paste::paste! {
                #[doc = "Create a `View` of [`<input type=\"" $type_name "\">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/" $type_name ") element."]
                #[inline]
                pub fn $type_name<TMsg: 'static>(attr: impl Attr<TMsg>) -> impl Node<Msg = TMsg> {
                    super::input((attribute("type", stringify!($type_name)), attr), ())
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
    pub fn datetime_local<TMsg: 'static>(attr: impl Attr<TMsg>) -> impl Node<Msg = TMsg> {
        super::input((attribute("type", "datetime-local"), attr), ())
    }
}

pub mod event {
    use serde::{de::IgnoredAny, Deserialize};
    use siro_vdom::attr::{event, Attr};

    pub fn on<T, TMsg>(event_type: &'static str, f: impl Fn(T) -> TMsg + 'static) -> impl Attr<TMsg>
    where
        T: for<'de> Deserialize<'de> + 'static,
        TMsg: 'static,
    {
        event(event_type, move |event| Some(f(event)))
    }

    macro_rules! define_events {
        ( $( $name:ident => $event_type:expr ),* $(,)? ) => {$(
            paste::paste! {
                #[inline]
                pub fn [< on_ $name >] <TMsg: 'static>(f: impl Fn() -> TMsg + 'static) -> impl Attr<TMsg> {
                    on($event_type, move |_: IgnoredAny| f())
                }
            }
        )*};
    }

    define_events! {
        click => "click",
        double_click => "dblclick",
        focus => "focus",
        blur => "blur",
    }

    #[derive(Deserialize)]
    struct InputEvent {
        target: InputTarget,
    }

    #[derive(Deserialize)]
    struct InputTarget {
        value: Option<String>,
        checked: Option<bool>,
    }

    pub fn on_input<TMsg: 'static>(f: impl Fn(String) -> TMsg + 'static) -> impl Attr<TMsg> {
        event("input", move |e: InputEvent| Some(f(e.target.value?)))
    }

    pub fn on_check<TMsg: 'static>(f: impl Fn(bool) -> TMsg + 'static) -> impl Attr<TMsg> {
        event("input", move |e: InputEvent| Some(f(e.target.checked?)))
    }

    #[derive(Deserialize)]
    struct KeyEvent {
        key: String,
    }

    pub fn on_enter<TMsg: 'static>(f: impl Fn() -> TMsg + 'static) -> impl Attr<TMsg> {
        event("keydown", move |e: KeyEvent| match &*e.key {
            "Enter" => Some(f()),
            _ => None,
        })
    }
}
