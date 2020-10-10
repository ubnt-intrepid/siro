use super::{
    id::{NodeId, NodeIdAnchor},
    node::VNode,
    types::{CowStr, FxIndexMap, FxIndexSet},
};
use gloo_events::EventListener;
use std::{
    fmt,
    hash::{Hash, Hasher},
};
use wasm_bindgen::JsValue;

/// A virtual [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) node.
#[non_exhaustive]
pub struct VElement {
    anchor: NodeIdAnchor,
    pub tag_name: CowStr,
    pub namespace_uri: Option<CowStr>,
    pub attributes: FxIndexMap<CowStr, Attribute>,
    pub properties: FxIndexMap<CowStr, Property>,
    pub listeners: FxIndexSet<Box<dyn Listener>>,
    pub classes: FxIndexSet<CowStr>,
    pub styles: FxIndexMap<CowStr, CowStr>,
    pub inner_html: Option<CowStr>,
    pub children: Vec<VNode>,
}

impl fmt::Debug for VElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VElement") //
            .field("tag_name", &self.tag_name)
            .field("namespace_uri", &self.namespace_uri)
            .field("attributes", &self.attributes)
            .field("properties", &self.properties)
            .field("listeners", &self.listeners)
            .field("classes", &self.classes)
            .field("styles", &self.styles)
            .field("inner_html", &self.inner_html)
            .field("children", &self.children)
            .finish()
    }
}

impl VElement {
    pub fn new(tag_name: CowStr, namespace_uri: Option<CowStr>) -> Self {
        Self {
            anchor: NodeIdAnchor::new(),
            tag_name,
            namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexSet::default(),
            classes: FxIndexSet::default(),
            styles: FxIndexMap::default(),
            inner_html: None,
            children: vec![],
        }
    }

    pub(crate) fn id(&self) -> &NodeId {
        self.anchor.id()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    String(CowStr),
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

impl From<CowStr> for Attribute {
    fn from(s: CowStr) -> Self {
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
    fn event_type(&self) -> &'static str;

    fn attach(&self, target: &web::EventTarget) -> EventListener;
}

impl fmt::Debug for dyn Listener + '_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("dyn_Listener")
            .field("event_type", &self.event_type())
            .finish()
    }
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
