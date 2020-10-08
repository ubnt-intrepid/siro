use std::{
    hash::{Hash, Hasher},
    rc::{Rc, Weak},
};

#[derive(Debug, Default)]
pub(super) struct NodeIdAnchor(Rc<()>);

impl NodeIdAnchor {
    #[inline]
    pub(super) fn id(&self) -> NodeId {
        NodeId(Rc::downgrade(&self.0))
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
