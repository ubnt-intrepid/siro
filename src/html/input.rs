use super::Element;
use crate::{
    mailbox::Mailbox,
    vdom::{Property, VElement, VNode},
};
use std::{borrow::Cow, marker::PhantomData};

macro_rules! input_elements {
    ($( $name:ident => $Type:ident, )*) => {$(
        pub fn $name() -> Input<$Type> {
            Input(VElement::new("input", None), PhantomData)
                .attribute("type", $Type::type_name())
        }
    )*};
}

input_elements! {
    text => Text,
    password => Password,
}

pub struct Input<Type: InputType = Text>(VElement, PhantomData<Type>);

impl<Type: InputType> From<Input<Type>> for VNode {
    fn from(e: Input<Type>) -> Self {
        e.0.into()
    }
}

impl<Type: InputType> Element for Input<Type> {
    fn as_velement(&mut self) -> &mut VElement {
        &mut self.0
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

    pub fn on_input<M, F, TMsg>(self, mailbox: &M, callback: F) -> Self
    where
        M: Mailbox<TMsg>,
        M::Sender: 'static,
        F: Fn(String) -> TMsg + 'static,
    {
        self.on_("input", mailbox, move |e| {
            Some(callback(
                #[allow(unused_unsafe)]
                unsafe {
                    js_sys::Reflect::get(&&e.target()?, &"value".into())
                        .ok()?
                        .as_string()?
                },
            ))
        })
    }
}

pub trait InputType {
    fn type_name() -> &'static str;
}

pub struct Text(std::convert::Infallible);
impl InputType for Text {
    fn type_name() -> &'static str {
        "text"
    }
}

pub struct Password(std::convert::Infallible);
impl InputType for Password {
    fn type_name() -> &'static str {
        "password"
    }
}
