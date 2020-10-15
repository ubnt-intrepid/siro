use super::{
    node::VNode,
    types::{CowStr, FxIndexMap, FxIndexSet},
};
use gloo_events::EventListener;
use std::fmt;
use wasm_bindgen::JsValue;

/// A virtual [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) node.
#[non_exhaustive]
pub struct VElement {
    pub(crate) node: web::Element,
    pub tag_name: CowStr,
    pub namespace_uri: Option<CowStr>,
    pub attributes: FxIndexMap<CowStr, Attribute>,
    pub properties: FxIndexMap<CowStr, Property>,
    pub listeners: FxIndexMap<CowStr, EventListener>,
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
    pub(crate) fn new(node: web::Element, tag_name: CowStr, namespace_uri: Option<CowStr>) -> Self {
        Self {
            node,
            tag_name,
            namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexMap::default(),
            classes: FxIndexSet::default(),
            styles: FxIndexMap::default(),
            inner_html: None,
            children: vec![],
        }
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
