use super::{cache::Key, listener::Listener, node::Node};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use wasm_bindgen::JsValue;

pub fn element(tag_name: &'static str) -> Element {
    Element {
        rc: Rc::new(()),
        tag_name,
        attrs: HashMap::new(),
        properties: HashMap::new(),
        listeners: HashSet::new(),
        children: vec![],
    }
}

pub struct Element {
    rc: Rc<()>,
    pub(super) tag_name: &'static str,
    pub(super) attrs: HashMap<String, String>,
    pub(super) properties: HashMap<String, Property>,
    pub(super) listeners: HashSet<Rc<dyn Listener>>,
    pub(super) children: Vec<Node>,
}

impl Element {
    pub(super) fn key(&self) -> Key {
        Key::new(&self.rc)
    }

    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attrs.insert(name.into(), value.into());
        self
    }

    pub fn property(mut self, name: &str, value: impl Into<Property>) -> Self {
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
