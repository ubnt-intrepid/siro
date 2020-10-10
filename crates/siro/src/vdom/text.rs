use super::{
    id::{NodeId, NodeIdAnchor},
    types::CowStr,
};
use std::fmt;

/// A virtual [`Text`](https://developer.mozilla.org/en-US/docs/Web/API/Text) node.
#[non_exhaustive]
pub struct VText {
    anchor: NodeIdAnchor,
    /// The content of this node.
    pub value: CowStr,
}

impl fmt::Debug for VText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VText") //
            .field("value", &self.value)
            .finish()
    }
}

impl VText {
    /// Create a new `VText` with the specified content.
    #[inline]
    pub fn new<S>(value: S) -> Self
    where
        S: Into<CowStr>,
    {
        Self {
            anchor: NodeIdAnchor::new(),
            value: value.into(),
        }
    }

    pub(crate) fn id(&self) -> &NodeId {
        self.anchor.id()
    }
}

macro_rules! impl_from_strs {
    ($( $t:ty ),*) => {$(
        impl From<$t> for VText {
            fn from(value: $t) -> Self {
                Self::new(value)
            }
        }
    )*};
}

impl_from_strs!(&'static str, String, CowStr);
