use crate::Meow;
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

    #[must_use]
    pub fn replace(&mut self, old: NodeId, new: NodeId) -> web::Node {
        let value = self.0.remove(&old).unwrap_throw();
        self.0.insert(new, value.clone());
        value
    }
}

pub enum Node {
    Text(Text),
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl Node {
    fn id(&self) -> NodeId {
        match self {
            Node::Text(t) => t.id,
        }
    }

    pub fn render(&self, meow: &Meow, caches: &mut NodeCaches) -> web::Node {
        match self {
            Node::Text(t) => {
                let node: web::Node = meow.document.create_text_node(&*t.value).into();
                caches.set(t.id, node.clone());
                node
            }
        }
    }

    pub fn diff(&self, new: &Node, _meow: &Meow, caches: &mut NodeCaches) {
        // Same nodes.
        if self.id() == new.id() {
            return;
        }

        let node = caches.replace(self.id(), new.id());

        match (self, new) {
            (Node::Text(current), Node::Text(new)) => {
                let node = node.dyn_ref::<web::Text>().unwrap_throw();
                if current.value != new.value {
                    node.set_data(&*new.value);
                }
            }
        }
    }
}

// ==== Text ====

pub struct Text {
    value: Cow<'static, str>,
    id: NodeId,
}
