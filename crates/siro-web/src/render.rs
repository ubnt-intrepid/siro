use gloo_events::EventListener;
use siro::{
    event::{Event, EventDecoder},
    node::{Attributes, AttributesRenderer, Nodes, NodesRenderer},
    subscription::{Mailbox as _, Subscriber},
    types::{Attribute, CowStr, Property},
};
use std::mem;
use wasm_bindgen::prelude::*;

// ==== VNode ====

type BuildFxHasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;
type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;
type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;

#[derive(Debug)]
pub(crate) enum VNode {
    Text(VText),
    Element(VElement),
}

impl VNode {
    fn as_node(&self) -> Option<&web::Node> {
        match self {
            VNode::Text(VText { node, .. }) => Some(node.as_ref()),
            VNode::Element(VElement { node, .. }) => Some(node.as_ref()),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VElement {
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    attributes: FxIndexMap<CowStr, Attribute>,
    properties: FxIndexMap<CowStr, Property>,
    listeners: FxIndexMap<CowStr, EventListener>,
    class_names: FxIndexSet<CowStr>,
    styles: FxIndexMap<CowStr, CowStr>,
    inner_html: Option<CowStr>,
    children: Vec<VNode>,
    node: web::Element,
}

impl VElement {
    fn apply_class(&self) -> Result<(), JsValue> {
        let class_name = self.class_names.iter().fold(String::new(), |mut acc, c| {
            if !acc.is_empty() {
                acc += " ";
            }
            acc += &*c;
            acc
        });
        set_attribute(&self.node, "class", &class_name.into())?;
        Ok(())
    }

    fn apply_style(&self) -> Result<(), JsValue> {
        let style = self
            .styles
            .iter()
            .fold(String::new(), |mut acc, (name, value)| {
                if !acc.is_empty() {
                    acc += ";";
                }
                acc += &*name;
                acc += ":";
                acc += &*value;
                acc
            });
        set_attribute(&self.node, "style", &style.into())?;
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct VText {
    data: CowStr,
    node: web::Text,
}

// ==== RenderContext ====

pub(super) struct RenderContext<'ctx, S: Subscriber> {
    pub(super) document: &'ctx web::Document,
    pub(super) parent: &'ctx web::Node,
    pub(super) subscriber: S,
}

impl<S: Subscriber> RenderContext<'_, S> {
    pub(super) fn diff_nodes<N>(&self, nodes: N, vnodes: &mut Vec<VNode>) -> Result<(), JsValue>
    where
        N: Nodes<S::Msg>,
    {
        nodes.render_nodes(DiffNodes {
            ctx: self,
            vnodes,
            num_children: 0,
        })?;

        Ok(())
    }

    fn reparent<'a>(&'a self, parent: &'a web::Node) -> RenderContext<'a, &'a S> {
        RenderContext {
            document: &*self.document,
            subscriber: &self.subscriber,
            parent,
        }
    }

    fn create_element<A, C>(
        &self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attrs: A,
        children: C,
    ) -> Result<VElement, JsValue>
    where
        A: Attributes<S::Msg>,
        C: Nodes<S::Msg>,
    {
        let node = match &namespace_uri {
            Some(uri) => self.document.create_element_ns(Some(&*uri), &*tag_name)?,
            None => self.document.create_element(&*tag_name)?,
        };

        let mut velement = VElement {
            node,
            tag_name,
            namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexMap::default(),
            class_names: FxIndexSet::default(),
            styles: FxIndexMap::default(),
            inner_html: None,
            children: vec![],
        };

        attrs.render_attributes(NewAttributes {
            ctx: self,
            velement: &mut velement,
        })?;

        if velement.inner_html.is_none() {
            self.reparent(&velement.node)
                .diff_nodes(children, &mut velement.children)?;
        }

        self.parent.append_child(&velement.node)?;

        Ok(velement)
    }

    fn create_text_node(&self, data: CowStr) -> Result<VText, JsValue> {
        let node = self.document.create_text_node(&*data);
        self.parent.append_child(&node)?;
        Ok(VText { node, data })
    }

    fn diff_element<A, C>(
        &self,
        vnode: &mut VNode,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attrs: A,
        children: C,
    ) -> Result<(), JsValue>
    where
        A: Attributes<S::Msg>,
        C: Nodes<S::Msg>,
    {
        match vnode {
            VNode::Element(velement)
                if velement.tag_name == tag_name && velement.namespace_uri == namespace_uri =>
            {
                let old_attributes = mem::take(&mut velement.attributes);
                let old_properties = mem::take(&mut velement.properties);
                let old_inner_html = velement.inner_html.take();
                velement.listeners.clear();
                velement.class_names.clear();
                velement.styles.clear();

                attrs.render_attributes(DiffAttributes {
                    ctx: self,
                    velement,
                    old_attributes,
                    old_properties,
                    old_inner_html,
                })?;

                if velement.inner_html.is_none() {
                    self.reparent(&velement.node)
                        .diff_nodes(children, &mut velement.children)?;
                }
            }

            _ => {
                let velement = self.create_element(tag_name, namespace_uri, attrs, children)?;
                *vnode = VNode::Element(velement);
            }
        }

        Ok(())
    }

    fn diff_text_node(&self, vnode: &mut VNode, data: CowStr) -> Result<(), JsValue> {
        match vnode {
            VNode::Text(t) => {
                if t.data != data {
                    t.node.set_data(&*data);
                    t.data = data;
                }
            }
            _ => {
                let vtext = self.create_text_node(data)?;
                *vnode = VNode::Text(vtext);
            }
        }

        Ok(())
    }
}

struct DiffNodes<'a, 'ctx, S: Subscriber> {
    ctx: &'a RenderContext<'ctx, S>,
    vnodes: &'a mut Vec<VNode>,
    num_children: usize,
}

impl<S: Subscriber> NodesRenderer for DiffNodes<'_, '_, S> {
    type Msg = S::Msg;
    type Ok = ();
    type Error = JsValue;

    fn element<A, C>(
        &mut self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
        attrs: A,
        children: C,
    ) -> Result<(), Self::Error>
    where
        A: Attributes<Self::Msg>,
        C: Nodes<Self::Msg>,
    {
        if let Some(vnode) = self.vnodes.get_mut(self.num_children) {
            self.ctx
                .diff_element(vnode, tag_name, namespace_uri, attrs, children)?;
        } else {
            let vnode = self
                .ctx
                .create_element(tag_name, namespace_uri, attrs, children)
                .map(VNode::Element)?;
            self.vnodes.push(vnode);
        }
        self.num_children += 1;
        Ok(())
    }

    fn text_node(&mut self, data: CowStr) -> Result<(), Self::Error> {
        if let Some(vnode) = self.vnodes.get_mut(self.num_children) {
            self.ctx.diff_text_node(vnode, data)?;
        } else {
            let vnode = self.ctx.create_text_node(data).map(VNode::Text)?;
            self.vnodes.push(vnode);
        }
        self.num_children += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        for vnode in self.vnodes.drain(self.num_children..) {
            if let Some(node) = vnode.as_node() {
                self.ctx.parent.remove_child(node)?;
            }
        }

        Ok(())
    }
}

struct DiffAttributes<'a, 'ctx, S: Subscriber> {
    ctx: &'a RenderContext<'ctx, S>,
    velement: &'a mut VElement,
    old_attributes: FxIndexMap<CowStr, Attribute>,
    old_properties: FxIndexMap<CowStr, Property>,
    old_inner_html: Option<CowStr>,
}

impl<S: Subscriber> AttributesRenderer for DiffAttributes<'_, '_, S> {
    type Msg = S::Msg;
    type Ok = ();
    type Error = JsValue;

    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        match self.old_attributes.remove(&*name) {
            Some(old_value) if old_value == value => (),
            _ => set_attribute(&self.velement.node, &*name, &value)?,
        }
        self.velement.attributes.insert(name, value);
        Ok(())
    }

    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        match self.old_properties.remove(&*name) {
            Some(old_value) if old_value == value => (),
            _ => set_property(&self.velement.node, &*name, &value)?,
        }
        self.velement.properties.insert(name, value);
        Ok(())
    }

    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        let mailbox = self.ctx.subscriber.mailbox();
        let listener = EventListener::new(&self.velement.node, event_type, move |event| {
            if let Some(msg) = decoder
                .decode_event(AppEvent { event })
                .expect_throw("failed to decode Event")
            {
                mailbox.send_message(msg);
            }
        });
        self.velement.listeners.insert(event_type.into(), listener);
        Ok(())
    }

    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        self.velement.class_names.insert(class_name);
        Ok(())
    }

    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        self.velement.styles.insert(name, value);
        Ok(())
    }

    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        match self.old_inner_html {
            Some(ref old) if *old == inner_html => (),
            _ => {
                self.velement.node.set_inner_html(&*inner_html);
            }
        }
        self.velement.inner_html.replace(inner_html);
        self.velement.children.clear();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        for (name, _) in self.old_attributes {
            self.velement.node.remove_attribute(&*name)?;
        }

        for (name, _) in self.old_properties {
            remove_property(&self.velement.node, &*name)?;
        }

        self.velement.apply_class()?;
        self.velement.apply_style()?;

        Ok(())
    }
}

// ==== NewAttributes ====

struct NewAttributes<'a, 'ctx, S: Subscriber> {
    ctx: &'a RenderContext<'ctx, S>,
    velement: &'a mut VElement,
}

impl<S: Subscriber> AttributesRenderer for NewAttributes<'_, '_, S> {
    type Msg = S::Msg;
    type Ok = ();
    type Error = JsValue;

    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        set_attribute(&self.velement.node, &*name, &value)?;
        self.velement.attributes.insert(name, value);
        Ok(())
    }

    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        set_property(&self.velement.node, &*name, &value)?;
        self.velement.properties.insert(name, value);
        Ok(())
    }

    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        let mailbox = self.ctx.subscriber.mailbox();
        let listener = EventListener::new(&self.velement.node, event_type, move |event| {
            if let Some(msg) = decoder
                .decode_event(AppEvent { event })
                .expect_throw("failed to decode Event")
            {
                mailbox.send_message(msg);
            }
        });

        self.velement.listeners.insert(event_type.into(), listener);

        Ok(())
    }

    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        self.velement.class_names.insert(class_name);
        Ok(())
    }

    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        self.velement.styles.insert(name, value);
        Ok(())
    }

    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        self.velement.node.set_inner_html(&*inner_html);
        self.velement.inner_html.replace(inner_html);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.velement.apply_class()?;
        self.velement.apply_style()?;
        Ok(())
    }
}

// ==== AppEvent ====

struct AppEvent<'a> {
    event: &'a web::Event,
}

impl Event for AppEvent<'_> {
    type Deserializer = serde_wasm_bindgen::Deserializer;
    type Error = serde_wasm_bindgen::Error;

    fn into_deserializer(self) -> Self::Deserializer {
        let value: &JsValue = self.event.as_ref();
        serde_wasm_bindgen::Deserializer::from(value.clone())
    }
}

// ==== utils ====

fn set_attribute(element: &web::Element, name: &str, value: &Attribute) -> Result<(), JsValue> {
    match value {
        Attribute::String(value) => element.set_attribute(name, value)?,
        Attribute::Bool(true) => element.set_attribute(name, "")?,
        Attribute::Bool(false) => element.remove_attribute(name)?,
    }
    Ok(())
}

fn set_property(element: &web::Element, name: &str, value: &Property) -> Result<(), JsValue> {
    let value = match value {
        Property::String(s) => JsValue::from_str(&*s),
        Property::Number(n) => JsValue::from_f64(*n),
        Property::Bool(b) => JsValue::from_bool(*b),
    };
    js_sys::Reflect::set(element, &JsValue::from_str(name), &value)?;
    Ok(())
}

fn remove_property(element: &web::Element, name: &str) -> Result<(), JsValue> {
    js_sys::Reflect::set(element, &JsValue::from_str(name), &JsValue::UNDEFINED)?;
    Ok(())
}