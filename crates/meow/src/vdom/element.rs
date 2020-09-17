use super::{cache::Key, listener::Listener, node::Node};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub fn element(tag_name: &'static str) -> Element {
    Element {
        rc: Rc::new(()),
        tag_name,
        attrs: HashMap::new(),
        listeners: HashSet::new(),
        children: vec![],
    }
}

pub struct Element {
    rc: Rc<()>,
    pub(super) tag_name: &'static str,
    pub(super) attrs: HashMap<String, String>,
    pub(super) listeners: HashSet<Rc<dyn Listener>>,
    pub(super) children: Vec<Node>,
}

impl Element {
    pub(super) fn key(&self) -> Key {
        Key::new(&self.rc)
    }

    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attrs.insert(name.into(), value.into());
        self
    }

    pub fn listener(mut self, listener: Rc<dyn Listener>) -> Self {
        self.listeners.replace(listener);
        self
    }

    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.children.push(child.into());
        self
    }
}
