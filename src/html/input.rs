use super::HtmlElement;
use crate::{
    builder::Element,
    event::HasInputEvent,
    vdom::{Property, VElement, VNode},
};
use std::{borrow::Cow, marker::PhantomData};

pub trait InputType {
    fn name() -> &'static str;
}

macro_rules! input_elements {
    ($( $name:ident => $Type:ident, )*) => {$(
        mod $name {
            pub struct $Type(std::convert::Infallible);

            impl super::InputType for $Type {
                fn name() -> &'static str {
                    stringify!($name)
                }
            }
        }

        pub type $Type = Input<$name::$Type>;

        impl HasInputEvent for $Type {}

        paste::paste! {
            #[doc = "Create a builder of [`<input type=\"" $name "\">`](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/" $name ") element."]
            #[inline]
            pub fn $name() -> $Type {
                Input::new()
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

pub struct Input<Type: InputType> {
    base: HtmlElement,
    _marker: PhantomData<Type>,
}

impl<Type: InputType> Input<Type> {
    fn new() -> Self {
        Self {
            base: HtmlElement::new("input".into()),
            _marker: PhantomData,
        }
        .attribute("type", Type::name())
    }
}

impl<Type: InputType> From<Input<Type>> for VNode {
    fn from(e: Input<Type>) -> Self {
        e.base.into()
    }
}

impl<Type: InputType> Element for Input<Type> {
    fn as_velement(&mut self) -> &mut VElement {
        self.base.as_velement()
    }
}

impl<Type: InputType> Input<Type> {
    pub fn disabled(self, value: bool) -> Self {
        self.attribute("disabled", value)
    }

    pub fn name(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("name", value.into())
    }

    pub fn placeholder(self, text: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("placeholder", text.into())
    }

    pub fn readonly(self, value: bool) -> Self {
        self.attribute("readonly", value)
    }

    pub fn value(self, value: impl Into<Property>) -> Self {
        self.property("value", value.into())
    }
}
