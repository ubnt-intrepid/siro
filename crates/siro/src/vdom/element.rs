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
    pub styles: FxIndexMap<Cow<'static, str>, Cow<'static, str>>,
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
            styles: FxIndexMap::default(),
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

pub trait Element: Into<VNode> {
    fn as_velement(&mut self) -> &mut VElement;

    fn attribute(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Attribute>,
    ) -> Self {
        self.as_velement()
            .attributes
            .insert(name.into(), value.into());
        self
    }

    fn property(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<Property>) -> Self {
        self.as_velement()
            .properties
            .insert(name.into(), value.into());
        self
    }

    fn listener(mut self, listener: Rc<dyn Listener>) -> Self {
        self.as_velement().listeners.replace(listener);
        self
    }

    fn child(mut self, child: impl Into<VNode>) -> Self {
        self.as_velement().children.push(child.into());
        self
    }

    fn children(mut self, children: impl Children) -> Self {
        children.assign(&mut self.as_velement().children);
        self
    }

    fn append(self, iter: impl IntoIterator<Item = impl Into<VNode>>) -> Self {
        struct IterChildren<I>(I);

        impl<I> Children for IterChildren<I>
        where
            I: IntoIterator,
            I::Item: Into<VNode>,
        {
            fn assign(self, children: &mut Vec<VNode>) {
                children.extend(self.0.into_iter().map(Into::into));
            }
        }

        self.children(IterChildren(iter))
    }

    fn class(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.as_velement().class_names.insert(value.into());
        self
    }

    fn style(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.as_velement().styles.insert(name.into(), value.into());
        self
    }

    fn id(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("id", value.into())
    }
}

impl Element for VElement {
    fn as_velement(&mut self) -> &mut VElement {
        self
    }
}

pub trait Children {
    fn assign(self, children: &mut Vec<VNode>)
    where
        Self: Sized;
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl< $H, $($T),+ > Children for ( $H, $($T),+ )
        where
            $H: Into<VNode>,
            $( $T: Into<VNode>, )+
        {
            fn assign(self, children: &mut Vec<VNode>) {
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
            fn assign(self, children: &mut Vec<VNode>) {
                children.push(self.0.into());
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);
