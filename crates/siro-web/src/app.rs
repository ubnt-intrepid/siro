use futures::{channel::mpsc, future::LocalBoxFuture, prelude::*, stream::FuturesUnordered};
use gloo_events::EventListener;
use siro::{
    event::{Event, EventDecoder},
    node::{ElementRenderer, IntoNode, Node, Renderer},
    subscription::{Mailbox, Subscriber, Subscription},
    types::{Attribute, CowStr, Property},
};
use std::mem;
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    document: web::Document,
    vnode: Option<VNode>,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
    pending_tasks: FuturesUnordered<LocalBoxFuture<'static, TMsg>>,
}

impl<TMsg: 'static> App<TMsg> {
    fn new(mountpoint: web::Node, document: web::Document) -> Self {
        let (tx, rx) = mpsc::unbounded();
        Self {
            mountpoint,
            document,
            vnode: None,
            tx,
            rx,
            pending_tasks: FuturesUnordered::new(),
        }
    }

    pub fn mount(selector: &str) -> Result<Self, JsValue> {
        let document = crate::document().ok_or("no Document exists")?;
        let mountpoint = document.query_selector(selector)?.ok_or("missing node")?;
        Ok(Self::new(mountpoint.into(), document))
    }

    pub fn mount_to_body() -> Result<Self, JsValue> {
        let document = crate::document().ok_or("no Document exists")?;
        let body = document.body().ok_or("missing body in document")?;
        Ok(Self::new(body.into(), document))
    }

    /// Register a `Subscription`.
    pub fn subscribe<S>(&self, subscription: S) -> Result<S::Subscribe, S::Error>
    where
        S: Subscription<Msg = TMsg>,
    {
        subscription.subscribe(AppSubscriber { tx: &self.tx })
    }

    pub fn spawn_local<Fut>(&mut self, future: Fut)
    where
        Fut: Future<Output = TMsg> + 'static,
    {
        self.pending_tasks.push(Box::pin(future));
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        futures::select! {
            ret = self.rx.next() => ret,
            msg = self.pending_tasks.select_next_some() => Some(msg),
            complete => None,
        }
    }

    pub fn render<N>(&mut self, node: N) -> Result<(), JsValue>
    where
        N: IntoNode<TMsg>,
    {
        let node = node.into_node();

        let vnode = node.render(AppRenderer {
            document: &self.document,
            tx: &self.tx,
            vnode: self.vnode.take(),
            parent: &self.mountpoint,
        })?;

        self.vnode.replace(vnode);

        Ok(())
    }
}

struct AppSubscriber<'a, TMsg: 'static> {
    tx: &'a mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Subscriber for AppSubscriber<'_, TMsg> {
    type Msg = TMsg;
    type Mailbox = AppMailbox<TMsg>;

    #[inline]
    fn mailbox(&self) -> Self::Mailbox {
        AppMailbox {
            tx: self.tx.clone(),
        }
    }
}

struct AppMailbox<TMsg> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Mailbox for AppMailbox<TMsg> {
    type Msg = TMsg;

    fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }
}

// ==== rendering context ====

type BuildFxHasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;
type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;
type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;

#[derive(Debug)]
enum VNode {
    Text(VText),
    Element(VElement),
    Unknown,
}

impl Default for VNode {
    fn default() -> Self {
        Self::Unknown
    }
}

impl VNode {
    fn as_node(&self) -> Option<&web::Node> {
        match self {
            VNode::Text(VText { node, .. }) => Some(node.as_ref()),
            VNode::Element(VElement { node, .. }) => Some(node.as_ref()),
            VNode::Unknown => None,
        }
    }
}

#[derive(Debug)]
struct VElement {
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
struct VText {
    data: CowStr,
    node: web::Text,
}

struct AppRenderer<'a, TMsg: 'static> {
    vnode: Option<VNode>,
    document: &'a web::Document,
    parent: &'a web::Node,
    tx: &'a mpsc::UnboundedSender<TMsg>,
}

impl<'a, TMsg: 'static> Renderer for AppRenderer<'a, TMsg> {
    type Msg = TMsg;
    type Ok = VNode;
    type Error = JsValue;

    type Element = AppElementRenderer<'a, TMsg>;

    fn element_node(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<Self::Element, Self::Error> {
        match self.vnode {
            Some(VNode::Element(mut velement))
                if velement.tag_name == tag_name && velement.namespace_uri == namespace_uri =>
            {
                let old_attributes = mem::take(&mut velement.attributes);
                let old_properties = mem::take(&mut velement.properties);
                let old_inner_html = velement.inner_html.take();
                velement.listeners.clear();
                velement.class_names.clear();
                velement.styles.clear();

                Ok(AppElementRenderer::Diff(DiffElement {
                    velement,
                    old_attributes,
                    old_properties,
                    old_inner_html,
                    document: self.document,
                    tx: self.tx,
                    num_new_children: 0,
                }))
            }

            _ => {
                let node = match &namespace_uri {
                    Some(uri) => self.document.create_element_ns(Some(&*uri), &*tag_name)?,
                    None => self.document.create_element(&*tag_name)?,
                };

                Ok(AppElementRenderer::New(NewElement {
                    velement: VElement {
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
                    },
                    document: self.document,
                    parent: self.parent,
                    tx: self.tx,
                }))
            }
        }
    }

    fn text_node(self, data: CowStr) -> Result<Self::Ok, Self::Error> {
        match self.vnode {
            Some(VNode::Text(mut t)) => {
                if t.data != data {
                    t.node.set_data(&*data);
                    t.data = data;
                }
                Ok(VNode::Text(t))
            }
            _ => {
                let node = self.document.create_text_node(&*data);
                self.parent.append_child(&node)?;
                Ok(VNode::Text(VText { node, data }))
            }
        }
    }
}

enum AppElementRenderer<'a, TMsg: 'static> {
    Diff(DiffElement<'a, TMsg>),
    New(NewElement<'a, TMsg>),
}

impl<TMsg: 'static> ElementRenderer for AppElementRenderer<'_, TMsg> {
    type Msg = TMsg;
    type Ok = VNode;
    type Error = JsValue;

    #[inline]
    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        match self {
            Self::Diff(me) => me.attribute(name, value),
            Self::New(me) => me.attribute(name, value),
        }
    }

    #[inline]
    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        match self {
            Self::Diff(me) => me.property(name, value),
            Self::New(me) => me.property(name, value),
        }
    }

    #[inline]
    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        match self {
            Self::Diff(me) => me.event(event_type, decoder),
            Self::New(me) => me.event(event_type, decoder),
        }
    }

    #[inline]
    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        match self {
            Self::Diff(me) => me.class(class_name),
            Self::New(me) => me.class(class_name),
        }
    }

    #[inline]
    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        match self {
            Self::Diff(me) => me.style(name, value),
            Self::New(me) => me.style(name, value),
        }
    }

    #[inline]
    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        match self {
            Self::Diff(me) => me.inner_html(inner_html),
            Self::New(me) => me.inner_html(inner_html),
        }
    }

    #[inline]
    fn child<T>(&mut self, child: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>,
    {
        match self {
            Self::Diff(me) => me.child(child),
            Self::New(me) => me.child(child),
        }
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Self::Diff(me) => me.end(),
            Self::New(me) => me.end(),
        }
    }
}

// ==== DiffElement ====

struct DiffElement<'a, TMsg: 'static> {
    velement: VElement,
    old_attributes: FxIndexMap<CowStr, Attribute>,
    old_properties: FxIndexMap<CowStr, Property>,
    old_inner_html: Option<CowStr>,
    document: &'a web::Document,
    tx: &'a mpsc::UnboundedSender<TMsg>,
    num_new_children: usize,
}

impl<TMsg: 'static> ElementRenderer for DiffElement<'_, TMsg> {
    type Msg = TMsg;
    type Ok = VNode;
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
        let tx = self.tx.clone();
        let listener = EventListener::new(&self.velement.node, event_type, move |event| {
            if let Some(msg) = decoder
                .decode_event(AppEvent { event })
                .expect_throw("failed to decode Event")
            {
                tx.unbounded_send(msg).unwrap_throw();
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

    fn child<T>(&mut self, child: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>,
    {
        if let Some(..) = self.velement.inner_html {
            return Ok(());
        }

        if let Some(slot) = self.velement.children.get_mut(self.num_new_children) {
            let vnode = child.render(AppRenderer {
                document: &*self.document,
                tx: &*self.tx,
                vnode: Some(mem::take(slot)),
                parent: &self.velement.node,
            })?;
            let _ = mem::replace(slot, vnode);
        } else {
            let vnode = child.render(AppRenderer {
                vnode: None,
                document: &*self.document,
                tx: &*self.tx,
                parent: &self.velement.node,
            })?;
            self.velement.children.push(vnode);
        }
        self.num_new_children += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        for (name, _) in self.old_attributes {
            self.velement.node.remove_attribute(&*name)?;
        }

        for (name, _) in self.old_properties {
            remove_property(&self.velement.node, &*name)?;
        }

        self.velement.apply_class()?;
        self.velement.apply_style()?;

        if self.velement.inner_html.is_none() {
            for child in self.velement.children.drain(self.num_new_children..) {
                if let Some(child) = child.as_node() {
                    self.velement.node.remove_child(child)?;
                }
            }
        }

        Ok(VNode::Element(self.velement))
    }
}

// ==== NewElement ====

struct NewElement<'a, TMsg: 'static> {
    velement: VElement,
    document: &'a web::Document,
    parent: &'a web::Node,
    tx: &'a mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> ElementRenderer for NewElement<'_, TMsg> {
    type Msg = TMsg;
    type Ok = VNode;
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
        let tx = self.tx.clone();
        let listener = EventListener::new(&self.velement.node, event_type, move |event| {
            if let Some(msg) = decoder
                .decode_event(AppEvent { event })
                .expect_throw("failed to decode Event")
            {
                tx.unbounded_send(msg).unwrap_throw();
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

    fn child<T>(&mut self, child: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>,
    {
        if self.velement.inner_html.is_none() {
            let child_vnode = child.render(AppRenderer {
                vnode: None,
                document: &*self.document,
                tx: &*self.tx,
                parent: &self.velement.node,
            })?;
            self.velement.children.push(child_vnode);
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.velement.apply_class()?;
        self.velement.apply_style()?;

        self.parent.append_child(&self.velement.node)?;

        Ok(VNode::Element(self.velement))
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
