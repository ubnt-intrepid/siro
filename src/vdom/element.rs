use super::{
    node::{Key, VNode},
    FxIndexMap, FxIndexSet,
};
use gloo_events::EventListener;
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    rc::Rc,
};
use wasm_bindgen::JsValue;

#[non_exhaustive]
pub struct VElement {
    rc: Rc<()>,
    pub tag_name: Cow<'static, str>,
    pub namespace_uri: Option<Cow<'static, str>>,
    pub attributes: FxIndexMap<Cow<'static, str>, Attribute>,
    pub properties: FxIndexMap<Cow<'static, str>, Property>,
    pub listeners: FxIndexSet<Rc<dyn Listener>>,
    pub class_names: FxIndexSet<Cow<'static, str>>,
    pub children: Vec<VNode>,
}

impl VElement {
    pub fn new(tag_name: Cow<'static, str>, namespace_uri: Option<Cow<'static, str>>) -> Self {
        Self {
            rc: Rc::new(()),
            tag_name,
            namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexSet::default(),
            class_names: FxIndexSet::default(),
            children: vec![],
        }
    }

    pub(super) fn key(&self) -> Key {
        Key::new(&self.rc)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    String(Cow<'static, str>),
    Bool(bool),
}

impl From<&'static str> for Attribute {
    fn from(s: &'static str) -> Self {
        Attribute::String(s.into())
    }
}

impl From<String> for Attribute {
    fn from(s: String) -> Self {
        Attribute::String(s.into())
    }
}

impl From<Cow<'static, str>> for Attribute {
    fn from(s: Cow<'static, str>) -> Self {
        Attribute::String(s)
    }
}

impl From<bool> for Attribute {
    fn from(b: bool) -> Self {
        Attribute::Bool(b)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Property {
    String(String),
    Bool(bool),
}

impl From<String> for Property {
    fn from(s: String) -> Self {
        Property::String(s)
    }
}

impl From<bool> for Property {
    fn from(b: bool) -> Self {
        Property::Bool(b)
    }
}

impl From<Property> for JsValue {
    fn from(property: Property) -> Self {
        match property {
            Property::String(s) => s.into(),
            Property::Bool(b) => b.into(),
        }
    }
}

pub trait Listener {
    fn event_type(&self) -> &str;

    fn attach(self: Rc<Self>, target: &web::EventTarget) -> EventListener;
}

impl PartialEq for dyn Listener + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.event_type() == other.event_type()
    }
}

impl Eq for dyn Listener + '_ {}

impl Hash for dyn Listener + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.event_type().hash(state)
    }
}
