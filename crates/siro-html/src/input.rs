//! Components for building [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) element.

use crate::html_element::HtmlElement;
use siro::{
    event::{EventHandler, EventHandlerBase},
    vdom::{Element, Property, VElement, VNode},
};
use std::borrow::Cow;

/// Represents builder of [`<input>`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input) elements.
pub trait Input: Element {
    const TYPE: &'static str;

    fn disabled(self, value: bool) -> Self {
        self.attribute("disabled", value)
    }

    fn name(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("name", value.into())
    }

    fn placeholder(self, text: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("placeholder", text.into())
    }

    fn readonly(self, value: bool) -> Self {
        self.attribute("readonly", value)
    }

    fn value(self, value: impl Into<Property>) -> Self {
        self.property("value", value.into())
    }
}

macro_rules! input_elements {
    ($( $name:ident => $Type:ident, )*) => {$(
        pub struct $Type(HtmlElement);

        impl Input for $Type {
            const TYPE: &'static str = stringify!($name);
        }

        impl From<$Type> for VNode {
            fn from(e: $Type) -> Self {
                e.0.into()
            }
        }

        impl Element for $Type {
            fn as_velement(&mut self) -> &mut VElement {
                self.0.as_velement()
            }
        }

        impl HasInputEvent for $Type {}

        paste::paste! {
            #[doc = "Create a builder of [`<input type=\"" $name "\">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/" $name ") element."]
            #[inline]
            pub fn $name() -> $Type {
                $Type(HtmlElement::new("input".into()))
                    .attribute("type", <$Type as Input>::TYPE)
            }
        }
    )*};
}

input_elements! {
    button => Button,
    checkbox => Checkbox,
    color => Color,
    date => Date,
    email => Email,
    image => Image,
    month => Month,
    number => Number,
    password => Password,
    radio => Radio,
    range => Range,
    search => Search,
    submit => Submit,
    tel => Tel,
    text => Text,
    time => Time,
    url => Url,
    week => Week,
}

// ==== on_input ====

/// A marker trait indicating that the element accepts the handler of [input events](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event).
pub trait HasInputEvent: Element {}

/// Create a handler of [input events](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event).
pub fn on_input<F, TMsg>(f: F) -> OnInput<F>
where
    F: Fn(String) -> TMsg, // FIXME: use InputEvent instead of String
    TMsg: 'static,
{
    OnInput { f }
}

pub struct OnInput<F> {
    f: F,
}

impl<F, TMsg> EventHandlerBase for OnInput<F>
where
    F: Fn(String) -> TMsg,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn event_type(&self) -> &'static str {
        "input"
    }

    fn invoke(&self, event: &web::Event) -> Option<Self::Msg> {
        Some((self.f)(
            js_sys::Reflect::get(&&event.target()?, &"value".into())
                .ok()?
                .as_string()?,
        ))
    }
}

impl<T, F, TMsg> EventHandler<T> for OnInput<F>
where
    T: HasInputEvent,
    F: Fn(String) -> TMsg,
    TMsg: 'static,
{
}
