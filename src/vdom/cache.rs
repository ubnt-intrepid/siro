use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};
use web_sys as web;

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Key(Weak<()>);

impl Key {
    pub(crate) fn new(rc: &Rc<()>) -> Self {
        Self(Rc::downgrade(rc))
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}

#[derive(Default)]
pub struct CachedNodes(HashMap<Key, web::Node>);

impl CachedNodes {
    pub fn set(&mut self, key: Key, node: web::Node) {
        self.0.insert(key, node);
    }

    pub fn remove(&mut self, key: Key) -> Option<web::Node> {
        self.0.remove(&key)
    }
}
