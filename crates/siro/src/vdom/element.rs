use super::{Context, CowStr, Node, NodeCache};
use crate::{
    attr::{self, Attr},
    children::{self, Children},
};
use gloo_events::EventListener;
use rustc_hash::FxHasher;
use std::{hash::BuildHasherDefault, marker::PhantomData};
use wasm_bindgen::JsValue;

type BuildFxHasher = BuildHasherDefault<FxHasher>;

type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;
type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;

/// Create a virtual node corresponding to an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element).
pub fn element<TMsg: 'static, A, C>(
    tag_name: impl Into<CowStr>,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
) -> Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Children<TMsg>,
{
    Element {
        tag_name: tag_name.into(),
        namespace_uri,
        attr,
        children,
        _marker: PhantomData,
    }
}

/// A virtual node that will be rendered as an [`Element`](https://developer.mozilla.org/en-US/docs/Web/API/Element).
pub struct Element<TMsg, A, C> {
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> Node for Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Children<TMsg>,
{
    type Msg = TMsg;
    type Cache = ElementCache;

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<Self::Cache, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        let node = ctx.create_element(&*self.tag_name, self.namespace_uri.as_deref())?;
        let classes = node.class_list();
        let style = js_sys::Reflect::get(&node, &JsValue::from_str("style"))?;

        let mut velement = ElementCache {
            node: node.clone(),
            tag_name: self.tag_name,
            namespace_uri: self.namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexMap::default(),
            classes: FxIndexSet::default(),
            styles: FxIndexMap::default(),
            inner_html: None,
            children: vec![],
        };

        self.attr.apply(&mut NewAttrs {
            ctx,
            velement: &mut velement,
            node: &node,
            classes: &classes,
            style: &style,
        })?;

        if let None = velement.inner_html {
            let mut cursor = 0;
            self.children.diff(&mut AppendChildren {
                ctx,
                vnodes: &mut velement.children,
                cursor: &mut cursor,
                parent: &node,
            })?;
        }

        Ok(velement)
    }

    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, cache: &mut Self::Cache) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        if cache.tag_name != self.tag_name || cache.namespace_uri != self.namespace_uri {
            let new = self.render(ctx)?;
            crate::util::replace_node(cache.as_ref(), new.as_ref())?;
            *cache = new;
            return Ok(());
        }

        {
            let classes = cache.node.class_list();
            let style = js_sys::Reflect::get(&cache.node, &JsValue::from_str("style"))?;

            let mut cx = DiffAttrs::new(&mut *ctx, cache, &classes, &style);
            self.attr.apply(&mut cx)?;
            cx.finish()?;
        }

        if let None = cache.inner_html {
            let mut cursor = 0;

            self.children.diff(&mut AppendChildren {
                ctx,
                vnodes: &mut cache.children,
                cursor: &mut cursor,
                parent: &cache.node,
            })?;

            for child in cache.children.drain(cursor..) {
                cache.node.remove_child((&*child).as_ref())?;
            }
        }

        Ok(())
    }
}

// === context types ====

struct NewAttrs<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
    velement: &'a mut ElementCache,
    node: &'a web::Element,
    classes: &'a web::DomTokenList,
    style: &'a JsValue,
}

impl<Ctx: ?Sized> attr::Context for NewAttrs<'_, Ctx>
where
    Ctx: Context,
{
    type Msg = Ctx::Msg;

    fn set_attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), JsValue> {
        set_attribute(&self.node, &*name, &value)?;
        self.velement.attributes.insert(name, value);
        Ok(())
    }

    fn set_property(&mut self, name: CowStr, value: Property) -> Result<(), JsValue> {
        set_property(&self.node, &*name, Some(value.clone()))?;
        self.velement.properties.insert(name, value);
        Ok(())
    }

    fn set_listener<F>(&mut self, event_type: &'static str, callback: F) -> Result<(), JsValue>
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static,
    {
        let listener = self
            .ctx
            .set_listener(self.node.as_ref(), event_type, callback);
        self.velement.listeners.insert(event_type.into(), listener);
        Ok(())
    }

    fn add_class(&mut self, name: CowStr) -> Result<(), JsValue> {
        self.classes.add_1(&*name)?;
        self.velement.classes.replace(name);
        Ok(())
    }

    fn add_style(&mut self, name: CowStr, value: CowStr) -> Result<(), JsValue> {
        js_sys::Reflect::set(
            &self.style,
            &JsValue::from_str(&*name),
            &JsValue::from_str(&*value),
        )?;
        self.velement.styles.insert(name, value);
        Ok(())
    }

    fn set_inner_html(&mut self, inner_html: CowStr) -> Result<(), JsValue> {
        self.node.set_inner_html(&*inner_html);
        self.velement.inner_html.replace(inner_html);
        Ok(())
    }
}

// FIXME: more efficient!

struct DiffAttrs<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
    old: &'a mut ElementCache,
    classes: &'a web::DomTokenList,
    style: &'a JsValue,
    new_attributes: FxIndexMap<CowStr, Attribute>,
    new_properties: FxIndexMap<CowStr, Property>,
    new_listeners: FxIndexMap<CowStr, EventListener>,
    new_classes: FxIndexSet<CowStr>,
    new_styles: FxIndexMap<CowStr, CowStr>,
}

impl<'a, Ctx: ?Sized> DiffAttrs<'a, Ctx>
where
    Ctx: Context,
{
    fn new(
        ctx: &'a mut Ctx,
        old: &'a mut ElementCache,
        classes: &'a web::DomTokenList,
        style: &'a JsValue,
    ) -> Self {
        Self {
            ctx,
            old,
            classes,
            style,
            new_attributes: FxIndexMap::default(),
            new_properties: FxIndexMap::default(),
            new_listeners: FxIndexMap::default(),
            new_classes: FxIndexSet::default(),
            new_styles: FxIndexMap::default(),
        }
    }

    fn finish(self) -> Result<(), JsValue> {
        let old_attributes = std::mem::replace(&mut self.old.attributes, self.new_attributes);
        for name in old_attributes.keys() {
            self.old.node.remove_attribute(name)?;
        }

        let old_properties = std::mem::replace(&mut self.old.properties, self.new_properties);
        for name in old_properties.keys() {
            set_property(&self.old.node, name, None)?;
        }

        let _ = std::mem::replace(&mut self.old.listeners, self.new_listeners);

        let old_classes = std::mem::replace(&mut self.old.classes, self.new_classes);
        for class in old_classes {
            self.classes.remove_1(&*class)?;
        }

        let old_styles = std::mem::replace(&mut self.old.styles, self.new_styles);
        for style in old_styles.keys() {
            js_sys::Reflect::set(&*self.style, &JsValue::from_str(style), &JsValue::UNDEFINED)?;
        }

        Ok(())
    }
}

impl<Ctx: ?Sized> attr::Context for DiffAttrs<'_, Ctx>
where
    Ctx: Context,
{
    type Msg = Ctx::Msg;

    fn set_attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), JsValue> {
        match self.old.attributes.remove(&name) {
            Some(old_value) if old_value == value => (),
            _ => set_attribute(&self.old.node, &name, &value)?,
        }
        self.new_attributes.insert(name, value);
        Ok(())
    }

    fn set_property(&mut self, name: CowStr, value: Property) -> Result<(), JsValue> {
        match self.old.properties.remove(&name) {
            Some(old_value) if old_value == value => (),
            _ => set_property(&self.old.node, &name, Some(value.clone()))?,
        }
        self.new_properties.insert(name, value);
        Ok(())
    }

    fn set_listener<F>(&mut self, event_type: &'static str, callback: F) -> Result<(), JsValue>
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static,
    {
        let listener = self
            .ctx
            .set_listener(self.old.node.as_ref(), event_type, callback);
        self.new_listeners.insert(event_type.into(), listener);
        Ok(())
    }

    fn add_class(&mut self, name: CowStr) -> Result<(), JsValue> {
        if !self.old.classes.remove(&name) {
            self.classes.add_1(&name)?;
        }
        self.new_classes.replace(name);
        Ok(())
    }

    fn add_style(&mut self, name: CowStr, value: CowStr) -> Result<(), JsValue> {
        match self.old.styles.remove(&name) {
            Some(old_value) if old_value == value => (),
            _ => {
                js_sys::Reflect::set(
                    &self.style,
                    &JsValue::from_str(&name),
                    &JsValue::from_str(&value),
                )?;
            }
        }
        self.new_styles.insert(name, value);
        Ok(())
    }

    fn set_inner_html(&mut self, inner_html: CowStr) -> Result<(), JsValue> {
        self.old.node.set_inner_html(&*inner_html);
        self.old.inner_html = Some(inner_html);
        self.old.children.clear();
        Ok(())
    }
}

fn set_attribute(element: &web::Element, name: &str, value: &Attribute) -> Result<(), JsValue> {
    match value {
        Attribute::String(value) => element.set_attribute(name, value)?,
        Attribute::Bool(true) => element.set_attribute(name, "")?,
        Attribute::Bool(false) => element.remove_attribute(name)?,
    }
    Ok(())
}

fn set_property(
    element: &web::Element,
    name: &str,
    value: Option<Property>,
) -> Result<(), JsValue> {
    js_sys::Reflect::set(element, &JsValue::from_str(name), &value.into())?;
    Ok(())
}

struct AppendChildren<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
    vnodes: &'a mut Vec<Box<dyn NodeCache>>,
    cursor: &'a mut usize,
    parent: &'a web::Element,
}

impl<Ctx: ?Sized> children::Context for AppendChildren<'_, Ctx>
where
    Ctx: Context,
{
    type Msg = Ctx::Msg;

    fn append_child<TNode>(&mut self, node: TNode) -> Result<(), JsValue>
    where
        TNode: Node<Msg = Self::Msg>,
    {
        if let Some(old) = self.vnodes.get_mut(*self.cursor) {
            super::diff(node, &mut *self.ctx, old)?;
        } else {
            let vnode = Node::render(node, &mut *self.ctx)?;
            self.parent.append_child(vnode.as_ref())?;
            self.vnodes.push(Box::new(vnode));
        }
        *self.cursor += 1;
        Ok(())
    }
}

pub struct ElementCache {
    node: web::Element,
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    attributes: FxIndexMap<CowStr, Attribute>,
    properties: FxIndexMap<CowStr, Property>,
    listeners: FxIndexMap<CowStr, EventListener>,
    classes: FxIndexSet<CowStr>,
    styles: FxIndexMap<CowStr, CowStr>,
    inner_html: Option<CowStr>,
    children: Vec<Box<dyn NodeCache>>,
}

impl AsRef<web::Node> for ElementCache {
    fn as_ref(&self) -> &web::Node {
        self.node.as_ref()
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
