use super::types::CowStr;
use std::fmt;

/// A virtual [`Text`](https://developer.mozilla.org/en-US/docs/Web/API/Text) node.
#[non_exhaustive]
pub struct VText {
    pub(crate) node: web::Text,
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
