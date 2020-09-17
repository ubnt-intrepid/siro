use crate::Meow;
use itertools::{EitherOrBoth, Itertools as _};
use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use web_sys as web;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct NodeId(Weak<()>);

impl PartialEq for NodeId {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for NodeId {}

impl Hash for NodeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

#[derive(Default)]
pub struct NodeCaches(HashMap<NodeId, web::Node>);

impl NodeCaches {
    pub fn set(&mut self, id: NodeId, node: web::Node) {
        self.0.insert(id, node);
    }

    pub fn remove(&mut self, id: NodeId) -> Option<web::Node> {
        self.0.remove(&id)
    }
}

pub enum Node {
    Element(Element),
    Text(Text),
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Self::Element(element)
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl Node {
    fn id(&self) -> NodeId {
        match self {
            Node::Element(e) => e.id(),
            Node::Text(t) => t.id(),
        }
    }

    pub fn render(&self, meow: &Meow, caches: &mut NodeCaches) -> Result<web::Node, JsValue> {
        match self {
            Node::Element(e) => {
                let element = meow
                    .document
                    .create_element(wasm_bindgen::intern(&*e.tag_name))?;

                for (name, value) in &e.attrs {
                    element.set_attribute(name, value)?;
                }

                for child in &e.children {
                    let child_element = child.render(meow, caches)?;
                    element.append_child(&child_element)?;
                }

                let node: web::Node = element.into();
                caches.set(e.id(), node.clone());
                Ok(node)
            }

            Node::Text(t) => {
                let node: web::Node = meow.document.create_text_node(&*t.value).into();
                caches.set(t.id(), node.clone());
                Ok(node)
            }
        }
    }

    pub fn diff(&self, new: &Node, meow: &Meow, caches: &mut NodeCaches) -> Result<(), JsValue> {
        if self.id() == new.id() {
            // Same nodes.
            return Ok(());
        }

        let node = caches
            .remove(self.id())
            .expect_throw("cache does not exist");
        caches.set(new.id(), node.clone());

        match (self, new) {
            (Node::Element(current), Node::Element(new)) if current.tag_name == new.tag_name => {
                let node = node
                    .dyn_ref::<web::Element>()
                    .expect_throw("cached node is not Element");

                for (name, new_value) in &new.attrs {
                    match current.attrs.get(name) {
                        Some(current) if current == new_value => (),
                        _ => {
                            node.set_attribute(name, new_value)?;
                        }
                    }
                }

                for name in current.attrs.keys() {
                    if !new.attrs.contains_key(name) {
                        node.remove_attribute(name)?;
                    }
                }

                for e in zip_longest(&current.children, &new.children) {
                    match e {
                        EitherOrBoth::Left(current) => {
                            let current = caches
                                .remove(current.id())
                                .expect_throw("cache does not exist");
                            node.remove_child(&current)?;
                        }
                        EitherOrBoth::Right(new) => {
                            let to_append = new.render(meow, caches)?;
                            node.append_child(&to_append)?;
                        }
                        EitherOrBoth::Both(current, new) => {
                            current.diff(new, meow, caches)?;
                        }
                    }
                }
            }

            (Node::Text(current), Node::Text(new)) => {
                let node = node
                    .dyn_ref::<web::Text>()
                    .expect_throw("cache is not Text");
                if current.value != new.value {
                    node.set_data(&*new.value);
                }
            }

            (_, new) => {
                if let Some(parent) = node.parent_node() {
                    let replacement = new.render(meow, caches)?;
                    parent.replace_child(&replacement, &node)?;
                }
            }
        }

        Ok(())
    }
}

// ==== Element ====

pub fn element(tag_name: &'static str) -> Element {
    Element {
        rc: Rc::new(()),
        tag_name,
        attrs: HashMap::new(),
        children: vec![],
    }
}

pub struct Element {
    rc: Rc<()>,
    tag_name: &'static str,
    attrs: HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
    fn id(&self) -> NodeId {
        NodeId(Rc::downgrade(&self.rc))
    }

    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attrs.insert(name.into(), value.into());
        self
    }

    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.children.push(child.into());
        self
    }
}

// ==== Text ====

pub fn text<S>(value: S) -> Text
where
    S: Into<Cow<'static, str>>,
{
    Text {
        rc: Rc::new(()),
        value: value.into(),
    }
}

pub struct Text {
    rc: Rc<()>,
    value: Cow<'static, str>,
}

impl Text {
    fn id(&self) -> NodeId {
        NodeId(Rc::downgrade(&self.rc))
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
