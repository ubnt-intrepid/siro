use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{Attribute, NodeId, Property, VElement, VNode},
    view::View,
};
use futures::{channel::mpsc, prelude::*};
use gloo_events::EventListener;
use itertools::{EitherOrBoth, Itertools as _};
use rustc_hash::FxHashMap;
use wasm_bindgen::{prelude::*, JsCast as _};

pub trait Mountpoint {
    fn get_node(&self, document: &web::Document) -> Result<web::Node, JsValue>;
}

impl Mountpoint for str {
    fn get_node(&self, document: &web::Document) -> Result<web::Node, JsValue> {
        document
            .query_selector(self)?
            .map(Into::into)
            .ok_or(format!("cannot find mountpoint: {}", self).into())
    }
}

impl Mountpoint for web::Node {
    fn get_node(&self, _: &web::Document) -> Result<web::Node, JsValue> {
        Ok(self.clone())
    }
}

impl Mountpoint for web::Element {
    fn get_node(&self, _: &web::Document) -> Result<web::Node, JsValue> {
        Ok(self.clone().into())
    }
}

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    vnode: VNode,
    renderer: Renderer,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg: 'static> App<TMsg> {
    pub fn mount(mountpoint: &(impl Mountpoint + ?Sized)) -> Result<Self, JsValue> {
        let document = crate::util::document().ok_or("no Document exists in Window")?;
        let mountpoint = mountpoint.get_node(&document)?;

        let mut renderer = Renderer::new(document);

        let view: VNode = "Now rendering...".into();
        let node = renderer.render(&view)?;
        mountpoint.append_child(&node)?;

        let (tx, rx) = mpsc::unbounded();

        Ok(App {
            mountpoint,
            vnode: view,
            renderer,
            tx,
            rx,
        })
    }

    pub fn mountpoint(&self) -> &web::Node {
        &self.mountpoint
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }

    pub fn render<TView>(&mut self, view: TView) -> Result<(), JsValue>
    where
        TView: View<Msg = TMsg>,
    {
        let vnode = view.render(&*self);

        self.renderer.diff(&self.vnode, &vnode)?;
        self.vnode = vnode;

        Ok(())
    }
}

impl<TMsg: 'static> Mailbox for App<TMsg> {
    type Msg = TMsg;
    type Sender = imp::AppSender<TMsg>;

    fn send_message(&self, msg: TMsg) {
        self.tx.unbounded_send(msg).unwrap_throw();
    }

    fn sender(&self) -> Self::Sender {
        imp::AppSender(self.tx.clone())
    }
}

mod imp {
    use super::*;

    pub struct AppSender<TMsg>(pub(super) mpsc::UnboundedSender<TMsg>);

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

struct Renderer {
    document: web::Document,
    cached_nodes: FxHashMap<NodeId, web::Node>,
    cached_listeners: FxHashMap<NodeId, FxHashMap<&'static str, EventListener>>,
}

impl Renderer {
    fn new(document: web::Document) -> Self {
        Self {
            document,
            cached_nodes: FxHashMap::default(),
            cached_listeners: FxHashMap::default(),
        }
    }

    pub(crate) fn render(&mut self, node: &VNode) -> Result<web::Node, JsValue> {
        let dom: web::Node = match node {
            VNode::Element(e) => self.render_element(e)?.into(),
            VNode::Text(t) => self.document.create_text_node(&*t.value).into(),
            VNode::Custom(n) => n.render(&self.document)?,
        };
        self.cached_nodes.insert(node.id(), dom.clone());
        Ok(dom)
    }

    fn render_element(&mut self, e: &VElement) -> Result<web::Element, JsValue> {
        let name = wasm_bindgen::intern(&*e.tag_name);
        let element = match e.namespace_uri {
            Some(ref uri) => self.document.create_element_ns(Some(&*uri), name)?,
            None => self.document.create_element(name)?,
        };

        for (name, value) in &e.attributes {
            set_attribute(&element, name, value)?;
        }

        for (name, value) in &e.properties {
            set_property(element.as_ref(), &*name, Some(value.clone()))?;
        }

        if !e.listeners.is_empty() {
            let caches = self.cached_listeners.entry(e.id()).or_default();

            for listener in &e.listeners {
                caches.insert(listener.event_type(), listener.attach(element.as_ref()));
            }
        }

        if !e.classes.is_empty() {
            let class_list = element.class_list();

            for class_name in &e.classes {
                class_list.add_1(&*class_name)?;
            }
        }

        if !e.styles.is_empty() {
            let style = js_sys::Reflect::get(&element, &"style".into())?;

            for (key, value) in &e.styles {
                let key = key.clone().into_owned();
                let value = value.clone().into_owned();
                js_sys::Reflect::set(&style, &key.into(), &value.into())?;
            }
        }

        for child in &e.children {
            let child_element = self.render(child)?;
            element.append_child(&child_element)?;
        }

        Ok(element)
    }

    pub(crate) fn diff(&mut self, old: &VNode, new: &VNode) -> Result<(), JsValue> {
        let old_key = old.id();
        let new_key = new.id();

        if old_key == new_key {
            // Same nodes.
            return Ok(());
        }

        let node = self.replant_caches(&old_key, &new_key);

        match (old, new) {
            (VNode::Element(old), VNode::Element(new)) if old.tag_name == new.tag_name => {
                let node = node
                    .dyn_ref::<web::Element>()
                    .expect_throw("cached node is not Element");
                self.diff_element(old, new, &node)?;
            }

            (VNode::Text(current), VNode::Text(new)) => {
                let node = node
                    .dyn_ref::<web::Text>()
                    .expect_throw("cache is not Text");
                if current.value != new.value {
                    node.set_data(&*new.value);
                }
            }

            (_, new) => {
                if let Some(parent) = node.parent_node() {
                    let replacement = self.render(new)?;
                    parent.replace_child(&replacement, &node)?;
                }
            }
        }

        Ok(())
    }

    fn replant_caches(&mut self, old: &NodeId, new: &NodeId) -> web::Node {
        let node = self
            .cached_nodes
            .remove(&old)
            .expect_throw("cache does not exist");
        self.cached_nodes.insert(new.clone(), node.clone());

        if let Some(listeners) = self.cached_listeners.remove(&old) {
            self.cached_listeners.insert(new.clone(), listeners);
        }

        node
    }

    fn diff_element(
        &mut self,
        old: &VElement,
        new: &VElement,
        node: &web::Element,
    ) -> Result<(), JsValue> {
        {
            for (name, new_value) in &new.attributes {
                match old.attributes.get(name) {
                    Some(old) if old == new_value => (),
                    _ => set_attribute(&node, name, new_value)?,
                }
            }

            for name in old.attributes.keys() {
                if !new.attributes.contains_key(name) {
                    node.remove_attribute(name)?;
                }
            }
        }

        {
            for (name, new_value) in &new.properties {
                match old.properties.get(name) {
                    Some(old) if old == new_value => (),
                    _ => {
                        set_property(node.as_ref(), &*name, Some(new_value.clone()))?;
                    }
                }
            }

            for name in old.properties.keys() {
                if !new.properties.contains_key(name) {
                    set_property(node.as_ref(), &*name, None)?;
                }
            }
        }

        {
            let caches = self.cached_listeners.entry(new.id()).or_default();

            caches.clear();

            for listener in &new.listeners {
                caches.insert(listener.event_type().into(), listener.attach(node.as_ref()));
            }
        }

        {
            let class_list = node.class_list();

            for added in new.classes.difference(&old.classes) {
                class_list.add_1(&*added)?
            }

            for removed in old.classes.difference(&new.classes) {
                class_list.remove_1(&*removed)?;
            }
        }

        {
            let style = js_sys::Reflect::get(&node, &"style".into())?;

            for (name, new_value) in &new.styles {
                match old.styles.get(name) {
                    Some(old) if old == new_value => (),
                    _ => {
                        let name = name.clone().into_owned();
                        let new_value = new_value.clone().into_owned();
                        js_sys::Reflect::set(&style, &name.into(), &new_value.into())?;
                    }
                }
            }

            for name in old.styles.keys() {
                if !new.styles.contains_key(name) {
                    let name = name.clone().into_owned();
                    js_sys::Reflect::set(&style, &name.into(), &JsValue::UNDEFINED)?;
                }
            }
        }

        for e in zip_longest(&old.children, &new.children) {
            match e {
                EitherOrBoth::Left(old) => {
                    let current = self
                        .cached_nodes
                        .remove(&old.id())
                        .expect_throw("cache does not exist");
                    node.remove_child(&current)?;
                }
                EitherOrBoth::Right(new) => {
                    let to_append = self.render(new)?;
                    node.append_child(&to_append)?;
                }
                EitherOrBoth::Both(old, new) => {
                    self.diff(old, new)?;
                }
            }
        }

        Ok(())
    }
}

#[inline]
fn zip_longest<I, J>(i: I, j: J) -> itertools::ZipLongest<I::IntoIter, J::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator,
{
    i.into_iter().zip_longest(j)
}
