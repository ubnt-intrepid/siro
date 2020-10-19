use futures::{channel::mpsc, prelude::*};
use gloo_events::EventListener;
use siro::{
    event::EventDecoder,
    mailbox::{Mailbox, Sender},
    node::{self, IntoNode, Node},
    subscription::Subscribe,
    types::{Attribute, CowStr, Property},
};
use wasm_bindgen::prelude::*;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    document: web::Document,
    vnode: Option<VNode>,
    mailbox: AppMailbox<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    fn new(mountpoint: web::Node, document: web::Document) -> Self {
        let (tx, rx) = mpsc::unbounded();
        Self {
            mountpoint,
            document,
            vnode: None,
            mailbox: AppMailbox { tx },
            rx,
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

    pub fn mailbox(&self) -> &impl Mailbox<Msg = TMsg> {
        &self.mailbox
    }

    /// Register a `Subscribe`.
    pub fn subscribe<S>(&self, s: S) -> Result<S::Subscription, S::Error>
    where
        S: Subscribe<Msg = TMsg>,
    {
        s.subscribe(&self.mailbox)
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }

    pub fn render<N>(&mut self, node: N) -> Result<(), JsValue>
    where
        N: IntoNode<TMsg>,
    {
        let node = node.into_node();
        node.render(RenderContext {
            document: &self.document,
            mailbox: &self.mailbox,
            parent: &self.mountpoint,
            vnode: self.vnode.get_or_insert(VNode::Null),
        })?;
        Ok(())
    }
}

struct AppMailbox<TMsg: 'static> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Mailbox for AppMailbox<TMsg> {
    type Msg = TMsg;
    type Sender = AppSender<TMsg>;

    fn send_message(&self, msg: TMsg) {
        self.tx.unbounded_send(msg).unwrap_throw();
    }

    fn sender(&self) -> Self::Sender {
        AppSender(self.tx.clone())
    }
}

struct AppSender<TMsg>(mpsc::UnboundedSender<TMsg>);

impl<TMsg> Clone for AppSender<TMsg> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<TMsg: 'static> Sender for AppSender<TMsg> {
    type Msg = TMsg;

    fn send_message(&self, msg: TMsg) {
        self.0.unbounded_send(msg).unwrap_throw();
    }
}

// ==== rendering context ====

type BuildFxHasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;
type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;
type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;

#[derive(Debug)]
enum VNode {
    Text {
        data: CowStr,
        node: web::Text,
    },
    Element {
        velement: VElement,
        node: web::Element,
    },
    Null,
}

impl VNode {
    fn as_node(&self) -> Option<&web::Node> {
        match self {
            VNode::Text { node, .. } => Some(node.as_ref()),
            VNode::Element { node, .. } => Some(node.as_ref()),
            VNode::Null => None,
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
}

impl VElement {
    fn new(tag_name: CowStr, namespace_uri: Option<CowStr>) -> Self {
        Self {
            tag_name,
            namespace_uri,
            attributes: FxIndexMap::default(),
            properties: FxIndexMap::default(),
            listeners: FxIndexMap::default(),
            class_names: FxIndexSet::default(),
            styles: FxIndexMap::default(),
            inner_html: None,
            children: vec![],
        }
    }
}

struct RenderContext<'a, TMsg: 'static> {
    document: &'a web::Document,
    mailbox: &'a AppMailbox<TMsg>,
    parent: &'a web::Node,
    vnode: &'a mut VNode,
}

impl<'a, TMsg: 'static> node::Context for RenderContext<'a, TMsg> {
    type Msg = TMsg;
    type Ok = ();
    type Error = JsValue;

    type Element = RenderElement<'a, TMsg>;

    fn element_node(
        self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<Self::Element, Self::Error> {
        let op = match self.vnode {
            VNode::Element { velement, .. }
                if velement.tag_name == tag_name && velement.namespace_uri == namespace_uri =>
            {
                RenderElementOp::Diff {
                    new_attributes: FxIndexMap::default(),
                    new_properties: FxIndexMap::default(),
                    new_listeners: FxIndexMap::default(),
                    new_class_names: FxIndexSet::default(),
                    new_styles: FxIndexMap::default(),
                    new_inner_html: None,
                    cursor: 0,
                }
            }
            _ => {
                let node = match &namespace_uri {
                    Some(uri) => self.document.create_element_ns(Some(&*uri), &*tag_name)?,
                    None => self.document.create_element(&*tag_name)?,
                };
                if let Some(old) = self.vnode.as_node() {
                    self.parent.replace_child(node.as_ref(), old)?;
                } else {
                    self.parent.append_child(node.as_ref())?;
                }
                let _old = std::mem::replace(
                    self.vnode,
                    VNode::Element {
                        node,
                        velement: VElement::new(tag_name, namespace_uri),
                    },
                );

                RenderElementOp::New
            }
        };

        match self.vnode {
            VNode::Element { node, velement, .. } => {
                let class_list = node.class_list();
                let style = js_sys::Reflect::get(&node, &JsValue::from_str("style"))?;

                Ok(RenderElement {
                    document: self.document,
                    mailbox: self.mailbox,
                    velement,
                    node,
                    class_list,
                    style,
                    op,
                })
            }

            _ => unreachable!("unexpected condition"),
        }
    }

    fn text_node(self, data: CowStr) -> Result<Self::Ok, Self::Error> {
        match self.vnode {
            VNode::Text {
                data: old_data,
                node,
                ..
            } => {
                if *old_data != data {
                    node.set_data(&*data);
                    *old_data = data;
                }
            }
            _ => {
                let node = self.document.create_text_node(&*data);
                if let Some(old) = self.vnode.as_node() {
                    self.parent.replace_child(node.as_ref(), old)?;
                } else {
                    self.parent.append_child(node.as_ref())?;
                }
                *self.vnode = VNode::Text { node, data };
            }
        }
        Ok(())
    }
}

struct RenderElement<'a, TMsg: 'static> {
    document: &'a web::Document,
    mailbox: &'a AppMailbox<TMsg>,
    velement: &'a mut VElement,
    node: &'a web::Element,
    class_list: web::DomTokenList,
    style: JsValue,
    op: RenderElementOp,
}

enum RenderElementOp {
    Diff {
        new_attributes: FxIndexMap<CowStr, Attribute>,
        new_properties: FxIndexMap<CowStr, Property>,
        new_listeners: FxIndexMap<CowStr, EventListener>,
        new_class_names: FxIndexSet<CowStr>,
        new_styles: FxIndexMap<CowStr, CowStr>,
        new_inner_html: Option<CowStr>,
        cursor: usize,
    },
    New,
}

impl<TMsg: 'static> node::ElementContext for RenderElement<'_, TMsg> {
    type Msg = TMsg;
    type Ok = ();
    type Error = JsValue;

    fn attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), Self::Error> {
        match &mut self.op {
            RenderElementOp::Diff { new_attributes, .. } => {
                match self.velement.attributes.remove(&*name) {
                    Some(old_value) if old_value == value => (),
                    _ => set_attribute(self.node, &*name, &value)?,
                }
                new_attributes.insert(name, value);
            }

            RenderElementOp::New => {
                set_attribute(self.node, &*name, &value)?;
                self.velement.attributes.insert(name, value);
            }
        }
        Ok(())
    }

    fn property(&mut self, name: CowStr, value: Property) -> Result<(), Self::Error> {
        match &mut self.op {
            RenderElementOp::Diff { new_properties, .. } => {
                match self.velement.properties.remove(&*name) {
                    Some(old_value) if old_value == value => (),
                    _ => set_property(self.node, &*name, &value)?,
                }
                new_properties.insert(name, value);
            }

            RenderElementOp::New => {
                set_property(self.node, &*name, &value)?;
                self.velement.properties.insert(name, value);
            }
        }
        Ok(())
    }

    fn event<D>(&mut self, event_type: &'static str, decoder: D) -> Result<(), Self::Error>
    where
        D: EventDecoder<Msg = Self::Msg> + 'static,
    {
        let sender = self.mailbox.sender();
        let listener = EventListener::new(self.node, event_type, move |event| {
            if let Some(msg) = decoder
                .decode_event(AppEvent { event })
                .expect_throw("failed to decode Event")
            {
                sender.send_message(msg);
            }
        });

        match &mut self.op {
            RenderElementOp::Diff { new_listeners, .. } => {
                new_listeners.insert(event_type.into(), listener);
            }
            RenderElementOp::New => {
                self.velement.listeners.insert(event_type.into(), listener);
            }
        };

        Ok(())
    }

    fn class(&mut self, class_name: CowStr) -> Result<(), Self::Error> {
        self.class_list.add_1(&*class_name)?;
        match &mut self.op {
            RenderElementOp::Diff {
                new_class_names, ..
            } => {
                self.velement.class_names.remove(&class_name);
                new_class_names.insert(class_name);
            }
            RenderElementOp::New => {
                self.velement.class_names.insert(class_name);
            }
        }
        Ok(())
    }

    fn style(&mut self, name: CowStr, value: CowStr) -> Result<(), Self::Error> {
        js_sys::Reflect::set(&self.style, &JsValue::from(&*name), &JsValue::from(&*value))?;

        match &mut self.op {
            RenderElementOp::Diff { new_styles, .. } => {
                self.velement.styles.remove(&name);
                new_styles.insert(name, value);
            }
            RenderElementOp::New => {
                self.velement.styles.insert(name, value);
            }
        }

        Ok(())
    }

    fn inner_html(&mut self, inner_html: CowStr) -> Result<(), Self::Error> {
        match &mut self.op {
            RenderElementOp::Diff { new_inner_html, .. } => {
                self.node.set_inner_html(&*inner_html);
                new_inner_html.replace(inner_html);
            }
            RenderElementOp::New => {
                self.node.set_inner_html(&*inner_html);
                self.velement.inner_html.replace(inner_html);
            }
        }

        Ok(())
    }

    fn child<T>(&mut self, child: T) -> Result<(), Self::Error>
    where
        T: Node<Msg = Self::Msg>,
    {
        match &mut self.op {
            RenderElementOp::Diff {
                new_inner_html: None,
                cursor,
                ..
            } => {
                self.velement.children.resize_with(
                    std::cmp::max(self.velement.children.len(), *cursor + 1),
                    || VNode::Null,
                );
                child.render(RenderContext {
                    document: &*self.document,
                    mailbox: &*self.mailbox,
                    parent: self.node.as_ref(),
                    vnode: &mut self.velement.children[*cursor],
                })?;
                *cursor += 1;
            }

            RenderElementOp::New if self.velement.inner_html.is_none() => {
                let mut vnode = VNode::Null;
                child.render(RenderContext {
                    document: &*self.document,
                    mailbox: &*self.mailbox,
                    parent: self.node.as_ref(),
                    vnode: &mut vnode,
                })?;
                self.velement.children.push(vnode);
            }

            _ => return Ok(()),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.op {
            RenderElementOp::Diff {
                new_attributes,
                new_properties,
                new_listeners,
                new_class_names,
                new_styles,
                new_inner_html,
                cursor,
                ..
            } => {
                let old_attributes =
                    std::mem::replace(&mut self.velement.attributes, new_attributes);
                let old_properties =
                    std::mem::replace(&mut self.velement.properties, new_properties);
                let _ = std::mem::replace(&mut self.velement.listeners, new_listeners);
                let old_class_names =
                    std::mem::replace(&mut self.velement.class_names, new_class_names);
                let old_styles = std::mem::replace(&mut self.velement.styles, new_styles);
                let _ = std::mem::replace(&mut self.velement.inner_html, new_inner_html);

                for (name, _) in old_attributes {
                    self.node.remove_attribute(&*name)?;
                }
                for (name, _) in old_properties {
                    remove_property(&self.node, &*name)?;
                }
                for name in old_class_names {
                    self.class_list.remove_1(&*name)?;
                }
                for (name, _) in old_styles {
                    js_sys::Reflect::set(&self.style, &JsValue::from(&*name), &JsValue::UNDEFINED)?;
                }

                if let Some(..) = self.velement.inner_html {
                    for child in self.velement.children.drain(..) {
                        if let Some(child) = child.as_node() {
                            self.node.remove_child(child)?;
                        }
                    }
                } else {
                    for child in self.velement.children.drain(cursor..) {
                        if let Some(child) = child.as_node() {
                            self.node.remove_child(child)?;
                        }
                    }
                }
            }

            RenderElementOp::New => {
                if !self.velement.class_names.is_empty() {
                    let class_list = self.node.class_list();
                    for class_name in &self.velement.class_names {
                        class_list.add_1(&*class_name)?;
                    }
                }

                if !self.velement.styles.is_empty() {
                    let style = js_sys::Reflect::get(&self.node, &JsValue::from_str("style"))?;
                    for (name, value) in &self.velement.styles {
                        js_sys::Reflect::set(
                            &style,
                            &JsValue::from_str(&*name),
                            &JsValue::from_str(&*value),
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

struct AppEvent<'a> {
    event: &'a web::Event,
}

mod impl_app_event {
    use super::*;
    use siro::event::Event;

    impl<'e> Event<'e> for AppEvent<'_> {
        type Deserializer = serde_wasm_bindgen::Deserializer;
        type Error = serde_wasm_bindgen::Error;

        fn into_deserializer(self) -> Self::Deserializer {
            let value: &JsValue = self.event.as_ref();
            serde_wasm_bindgen::Deserializer::from(value.clone())
        }
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
    let value = match value.clone() {
        Property::String(s) => s.into(),
        Property::Bool(b) => b.into(),
    };
    js_sys::Reflect::set(element, &JsValue::from_str(name), &value)?;
    Ok(())
}

fn remove_property(element: &web::Element, name: &str) -> Result<(), JsValue> {
    js_sys::Reflect::set(element, &JsValue::from_str(name), &JsValue::UNDEFINED)?;
    Ok(())
}