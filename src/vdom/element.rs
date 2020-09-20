use super::{cache::Key, node::Node};
use crate::util::{FxIndexMap, FxIndexSet};
use gloo_events::EventListener;
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    rc::Rc,
};
use wasm_bindgen::JsValue;

pub fn element(tag_name: &'static str, namespace: Option<&'static str>) -> Element {
    Element {
        rc: Rc::new(()),
        tag_name,
        namespace_uri: namespace,
        attributes: FxIndexMap::default(),
        properties: FxIndexMap::default(),
        listeners: FxIndexSet::default(),
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
    pub(super) attributes: FxIndexMap<Cow<'static, str>, Attribute>,
    pub(super) properties: FxIndexMap<Cow<'static, str>, Property>,
    pub(super) listeners: FxIndexSet<Rc<dyn Listener>>,
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

    pub fn children(mut self, children: impl Children) -> Self {
        children.assign(&mut self.children);
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

pub trait Children {
    fn assign(self, children: &mut Vec<Node>)
    where
        Self: Sized;
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl< $H, $($T),+ > Children for ( $H, $($T),+ )
        where
            $H: Into<Node>,
            $( $T: Into<Node>, )+
        {
            fn assign(self, children: &mut Vec<Node>) {
                #[allow(non_snake_case)]
                let ( $H, $($T),+ ) = self;

                children.push($H.into());
                $( children.push($T.into()); )+
            }
        }

        impl_children_for_tuples!( $($T),+ );
    };

    ( $C:ident ) => {
        impl< $C > Children for ( $C, )
        where
            $C: Into<Node>,
        {
            fn assign(self, children: &mut Vec<Node>) {
                children.push(self.0.into());
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);
