/*!
HTML directives for `siro`.
!*/

#![doc(html_root_url = "https://docs.rs/siro-html/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

use siro::{
    node::{Attributes, Nodes, NodesRenderer},
    types::CowStr,
};
use std::marker::PhantomData;

fn html_element<TMsg: 'static, A, C>(
    tag_name: impl Into<CowStr>,
    attr: A,
    children: C,
) -> HtmlElement<TMsg, A, C>
where
    A: Attributes<TMsg>,
    C: Nodes<TMsg>,
{
    HtmlElement {
        tag_name: tag_name.into(),
        attr,
        children,
        _marker: PhantomData,
    }
}

struct HtmlElement<TMsg, A, C> {
    tag_name: CowStr,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> Nodes<TMsg> for HtmlElement<TMsg, A, C>
where
    A: Attributes<TMsg>,
    C: Nodes<TMsg>,
{
    fn render_nodes<R>(self, mut renderer: R) -> Result<R::Ok, R::Error>
    where
        R: NodesRenderer<Msg = TMsg>,
    {
        renderer.element(self.tag_name, None, self.attr, self.children)?;
        renderer.end()
    }
}

macro_rules! html_elements {
    ( $( $tag_name:ident ),* $(,)? ) => {$(
        paste::paste! {
            #[doc = "Create a `View` of [`<" $tag_name ">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/" $tag_name ") element."]
            #[inline]
            pub fn $tag_name<TMsg: 'static>(
                attributes: impl Attributes<TMsg>,
                children: impl Nodes<TMsg>
            ) -> impl Nodes<TMsg> {
                html_element(stringify!($tag_name), attributes, children)
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
        attr::{attribute, property},
        node::Attributes,
        types::{CowStr, Property},
    };

    pub fn autofocus<TMsg: 'static>(autofocus: bool) -> impl Attributes<TMsg> {
        attribute("autofocus", autofocus)
    }

    pub fn href<TMsg: 'static>(url: impl Into<CowStr>) -> impl Attributes<TMsg> {
        attribute("href", url.into())
    }

    pub fn id<TMsg: 'static>(id: impl Into<CowStr>) -> impl Attributes<TMsg> {
        attribute("id", id.into())
    }

    pub fn label_for<TMsg: 'static>(target_id: impl Into<CowStr>) -> impl Attributes<TMsg> {
        attribute("for", target_id.into())
    }

    pub fn name<TMsg: 'static>(name: impl Into<CowStr>) -> impl Attributes<TMsg> {
        attribute("name", name.into())
    }

    pub fn placeholder<TMsg: 'static>(placeholder: impl Into<CowStr>) -> impl Attributes<TMsg> {
        attribute("placeholder", placeholder.into())
    }

    pub fn checked<TMsg: 'static>(checked: bool) -> impl Attributes<TMsg> {
        property("checked", checked)
    }

    pub fn value<TMsg: 'static>(value: impl Into<Property>) -> impl Attributes<TMsg> {
        property("value", value)
    }
}

/// `View`s for [`<input>`] with specific element type.
///
/// [`<input>`]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input
pub mod input {
    use siro::{
        attr::attribute,
        node::{Attributes, Nodes},
    };

    macro_rules! input_elements {
        ( $( $type_name:ident ),* $(,)? ) => {$(
            paste::paste! {
                #[doc = "Create a `View` of [`<input type=\"" $type_name "\">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/" $type_name ") element."]
                #[inline]
                pub fn $type_name<TMsg: 'static>(attr: impl Attributes<TMsg>) -> impl Nodes<TMsg> {
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
    pub fn datetime_local<TMsg: 'static>(attr: impl Attributes<TMsg>) -> impl Nodes<TMsg> {
        super::input((attribute("type", "datetime-local"), attr), ())
    }
}

pub mod event {
    use serde::{de::IgnoredAny, Deserialize};
    use siro::{attr::event, node::Attributes};

    pub fn on<T, TMsg>(
        event_type: &'static str,
        f: impl Fn(T) -> TMsg + 'static,
    ) -> impl Attributes<TMsg>
    where
        T: for<'de> Deserialize<'de> + 'static,
        TMsg: 'static,
    {
        event(event_type, move |event| Some(f(event)))
    }

    macro_rules! define_events {
        ( $( $name:ident => $event_type:expr ),* $(,)? ) => {$(
            #[inline]
            pub fn $name<TMsg: 'static>(f: impl Fn() -> TMsg + 'static) -> impl Attributes<TMsg> {
                on($event_type, move |_: IgnoredAny| f())
            }
        )*};
    }

    define_events! {
        on_click => "click",
        on_double_click => "dblclick",
        on_focus => "focus",
        on_blur => "blur",
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

    pub fn on_input<TMsg: 'static>(f: impl Fn(String) -> TMsg + 'static) -> impl Attributes<TMsg> {
        event("input", move |e: InputEvent| Some(f(e.target.value?)))
    }

    pub fn on_check<TMsg: 'static>(f: impl Fn(bool) -> TMsg + 'static) -> impl Attributes<TMsg> {
        event("input", move |e: InputEvent| Some(f(e.target.checked?)))
    }

    #[derive(Deserialize)]
    struct KeyEvent {
        key: String,
    }

    pub fn on_enter<TMsg: 'static>(f: impl Fn() -> TMsg + 'static) -> impl Attributes<TMsg> {
        event("keydown", move |e: KeyEvent| match &*e.key {
            "Enter" => Some(f()),
            _ => None,
        })
    }
}
