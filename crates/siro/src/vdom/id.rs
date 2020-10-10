use std::{
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub(super) struct NodeIdAnchor {
    rc: Rc<()>,
    id: NodeId,
}

impl NodeIdAnchor {
    pub(super) fn new() -> Self {
        let rc = Rc::new(());
        let id = NodeId(Rc::downgrade(&rc));
        Self { rc, id }
    }

    #[inline]
    pub(super) fn id(&self) -> &NodeId {
        &self.id
    }
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub(crate) struct NodeId(Weak<()>);

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
