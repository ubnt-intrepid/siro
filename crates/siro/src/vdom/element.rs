use super::{
    node::{Id, VNode},
    types::{CowStr, FxIndexMap, FxIndexSet},
};
use gloo_events::EventListener;
use std::{
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};
use wasm_bindgen::JsValue;

/// A virtual [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element) node.
#[non_exhaustive]
pub struct VElement {
    rc: Rc<()>,
    pub tag_name: CowStr,
    pub namespace_uri: Option<CowStr>,
    pub attributes: FxIndexMap<CowStr, Attribute>,
    pub properties: FxIndexMap<CowStr, Property>,
    pub listeners: FxIndexSet<Box<dyn Listener>>,
    pub classes: FxIndexSet<CowStr>,
    pub styles: FxIndexMap<CowStr, CowStr>,
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
            .field("children", &self.children)
            .finish()
    }
}

impl VElement {
    pub fn new(tag_name: CowStr, namespace_uri: Option<CowStr>) -> Self {
        Self {
            rc: Rc::new(()),
            tag_name,
            namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexSet::default(),
            classes: FxIndexSet::default(),
            styles: FxIndexMap::default(),
            children: vec![],
        }
    }

    pub(super) fn id(&self) -> Id {
        Id::new(&self.rc)
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

/// A mix-in trait for building virtual `Element`.
pub trait Element: Sized {
    /// Return a mutable reference to the target `VElement` to modify.
    fn as_velement(&mut self) -> &mut VElement;

    fn attribute(mut self, name: impl Into<CowStr>, value: impl Into<Attribute>) -> Self {
        self.as_velement()
            .attributes
            .insert(name.into(), value.into());
        self
    }

    fn property(mut self, name: impl Into<CowStr>, value: impl Into<Property>) -> Self {
        self.as_velement()
            .properties
            .insert(name.into(), value.into());
        self
    }

    fn listener(mut self, listener: Box<dyn Listener>) -> Self {
        self.as_velement().listeners.replace(listener);
        self
    }

    /// Specify a `class` to this element.
    fn class(mut self, value: impl Into<CowStr>) -> Self {
        self.as_velement().classes.insert(value.into());
        self
    }

    /// Specify an inline style to this element.
    ///
    /// The style will be set as a field of [`style`](https://developer.mozilla.org/en-US/docs/Web/API/ElementCSSInlineStyle/style) property.
    fn style(mut self, name: impl Into<CowStr>, value: impl Into<CowStr>) -> Self {
        self.as_velement().styles.insert(name.into(), value.into());
        self
    }

    /// Specify the [`id`](https://developer.mozilla.org/en-US/docs/Web/API/Element/id) property of this element.
    fn id(self, value: impl Into<CowStr>) -> Self {
        self.attribute("id", value.into())
    }

    /// Append a child node to this element.
    fn child(mut self, child: impl Into<VNode>) -> Self {
        self.as_velement().children.push(child.into());
        self
    }

    /// Append a set of child nodes to this element.
    fn children(mut self, children: impl Children) -> Self {
        children.append_to(&mut self.as_velement().children);
        self
    }

    /// Append an iterator of child nodes to this element.
    fn append(self, iter: impl IntoIterator<Item = impl Into<VNode>>) -> Self {
        self.children(IterChildren(iter))
    }
}

impl Element for VElement {
    fn as_velement(&mut self) -> &mut VElement {
        self
    }
}

/// A trait that represents a set of child nodes.
pub trait Children {
    /// Append itself to `children`.
    fn append_to(self, children: &mut Vec<VNode>)
    where
        Self: Sized;
}

impl Children for () {
    fn append_to(self, _: &mut Vec<VNode>) {}
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl< $H, $($T),+ > Children for ( $H, $($T),+ )
        where
            $H: Into<VNode>,
            $( $T: Into<VNode>, )+
        {
            fn append_to(self, children: &mut Vec<VNode>) {
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
            $C: Into<VNode>,
        {
            fn append_to(self, children: &mut Vec<VNode>) {
                children.push(self.0.into());
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);

struct IterChildren<I>(I);

impl<I> Children for IterChildren<I>
where
    I: IntoIterator,
    I::Item: Into<VNode>,
{
    fn append_to(self, children: &mut Vec<VNode>) {
        children.extend(self.0.into_iter().map(Into::into));
    }
}
