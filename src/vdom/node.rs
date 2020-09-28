use super::{element::VElement, text::VText};
use std::{
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub(super) struct Key(Weak<()>);

impl Key {
    pub(super) fn new(rc: &Rc<()>) -> Self {
        Self(Rc::downgrade(rc))
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

pub enum VNode {
    Element(VElement),
    Text(VText),
    Custom(CustomNode),
}

impl From<VElement> for VNode {
    fn from(element: VElement) -> Self {
        Self::Element(element)
    }
}

impl From<VText> for VNode {
    fn from(text: VText) -> Self {
        Self::Text(text)
    }
}

impl From<CustomNode> for VNode {
    fn from(custom: CustomNode) -> Self {
        Self::Custom(custom)
    }
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for VNode {
            fn from(value: $t) -> Self {
                Self::Text(VText::from(value))
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, std::borrow::Cow<'static, str>);

impl VNode {
    pub(super) fn key(&self) -> Key {
        match self {
            VNode::Element(e) => e.key(),
            VNode::Text(t) => t.key(),
            VNode::Custom(n) => n.key(),
        }
    }
}

pub struct CustomNode {
    rc: Rc<()>,
    render: Box<dyn Fn(&web::Document) -> Result<web::Node, JsValue>>,
}

impl CustomNode {
    pub fn new<F>(render: F) -> Self
    where
        F: Fn(&web::Document) -> Result<web::Node, JsValue> + 'static,
    {
        Self {
            rc: Rc::new(()),
            render: Box::new(render),
        }
    }

    fn key(&self) -> Key {
        Key::new(&self.rc)
    }

    pub(super) fn render(&self, document: &web::Document) -> Result<web::Node, JsValue> {
        (self.render)(document)
    }
}
