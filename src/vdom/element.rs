use super::{cache::Key, node::Node};
use gloo_events::EventListener;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    rc::Rc,
};
use wasm_bindgen::JsValue;
use web_sys as web;

pub fn element(tag_name: &'static str, namespace: Option<&'static str>) -> Element {
    Element {
        rc: Rc::new(()),
        tag_name,
        namespace_uri: namespace,
        attributes: HashMap::new(),
        properties: HashMap::new(),
        listeners: HashSet::new(),
        children: vec![],
    }
}

pub fn html(name: &'static str) -> Element {
    element(name, None)
}

pub fn svg(name: &'static str) -> Element {
    element(name, Some("http://www.w3.org/2000/svg"))
}

pub struct Element {
    rc: Rc<()>,
    pub(super) tag_name: &'static str,
    pub(super) namespace_uri: Option<&'static str>,
    pub(super) attributes: HashMap<Cow<'static, str>, Attribute>,
    pub(super) properties: HashMap<Cow<'static, str>, Property>,
    pub(super) listeners: HashSet<Rc<dyn Listener>>,
    pub(super) children: Vec<Node>,
}

impl Element {
    pub(super) fn key(&self) -> Key {
        Key::new(&self.rc)
    }

    pub fn attribute(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Attribute>,
    ) -> Self {
        self.attributes.insert(name.into(), value.into());
        self
    }

    pub fn property(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Property>,
    ) -> Self {
        self.properties.insert(name.into(), value.into());
        self
    }

    pub fn listener(mut self, listener: Rc<dyn Listener>) -> Self {
        self.listeners.replace(listener);
        self
    }

    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.children.push(child.into());
        self
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
