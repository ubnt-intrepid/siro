use crate::Meow;
use itertools::{EitherOrBoth, Itertools as _};
use std::{borrow::Cow, cell::Cell, collections::HashMap};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use web_sys as web;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeId(u32);

#[derive(Debug, Default)]
pub struct NodeFactory(Cell<u32>);

impl NodeFactory {
    fn new_id(&self) -> NodeId {
        let new_id = self.0.get();
        self.0.set(new_id + 1);
        NodeId(new_id)
    }

    pub fn element(&self, tag_name: &'static str) -> Element {
        Element {
            id: self.new_id(),
            tag_name,
            attrs: HashMap::new(),
            children: vec![],
        }
    }

    pub fn text<S>(&self, value: S) -> Text
    where
        S: Into<Cow<'static, str>>,
    {
        Text {
            id: self.new_id(),
            value: value.into(),
        }
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

    #[must_use]
    pub fn replace(&mut self, old: NodeId, new: NodeId) -> web::Node {
        let value = self.0.remove(&old).unwrap_throw();
        self.0.insert(new, value.clone());
        value
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
            Node::Element(e) => e.id,
            Node::Text(t) => t.id,
        }
    }

    pub fn render(&self, meow: &Meow, caches: &mut NodeCaches) -> web::Node {
        match self {
            Node::Element(e) => {
                let element = meow
                    .document
                    .create_element(wasm_bindgen::intern(&*e.tag_name))
                    .unwrap_throw();

                for (name, value) in &e.attrs {
                    element.set_attribute(name, value).unwrap_throw();
                }

                for child in &e.children {
                    let child_element = child.render(meow, caches);
                    element.append_child(&child_element).unwrap_throw();
                }

                let node: web::Node = element.into();
                caches.set(e.id, node.clone());
                node
            }

            Node::Text(t) => {
                let node: web::Node = meow.document.create_text_node(&*t.value).into();
                caches.set(t.id, node.clone());
                node
            }
        }
    }

    pub fn diff(&self, new: &Node, meow: &Meow, caches: &mut NodeCaches) {
        // Same nodes.
        if self.id() == new.id() {
            return;
        }

        let node = caches.replace(self.id(), new.id());

        match (self, new) {
            (Node::Element(current), Node::Element(new)) if current.tag_name == new.tag_name => {
                let node = node.dyn_ref::<web::Element>().unwrap_throw();

                for (name, new_value) in &new.attrs {
                    match current.attrs.get(name) {
                        Some(current) if current == new_value => (),
                        _ => {
                            node.set_attribute(name, new_value).unwrap_throw();
                        }
                    }
                }

                for name in current.attrs.keys() {
                    if !new.attrs.contains_key(name) {
                        node.remove_attribute(name).unwrap_throw();
                    }
                }

                for e in zip_longest(&current.children, &new.children) {
                    match e {
                        EitherOrBoth::Left(current) => {
                            let current = caches.remove(current.id()).unwrap_throw();
                            node.remove_child(&current).unwrap_throw();
                        }
                        EitherOrBoth::Right(new) => {
                            let to_append = new.render(meow, caches);
                            node.append_child(&to_append).unwrap_throw();
                        }
                        EitherOrBoth::Both(current, new) => {
                            current.diff(new, meow, caches);
                        }
                    }
                }
            }

            (Node::Text(current), Node::Text(new)) => {
                let node = node.dyn_ref::<web::Text>().unwrap_throw();
                if current.value != new.value {
                    node.set_data(&*new.value);
                }
            }

            (_, new) => {
                let replacement = new.render(meow, caches);
                if let Some(parent) = node.parent_node() {
                    parent.replace_child(&replacement, &node).unwrap_throw();
                }
            }
        }
    }
}

// ==== Element ====

pub struct Element {
    id: NodeId,
    tag_name: &'static str,
    attrs: HashMap<String, String>,
    children: Vec<Node>,
}

impl Element {
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

pub struct Text {
    value: Cow<'static, str>,
    id: NodeId,
}

#[inline]
fn zip_longest<I, J>(i: I, j: J) -> itertools::ZipLongest<I::IntoIter, J::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator,
{
    i.into_iter().zip_longest(j)
}
