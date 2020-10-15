//! A virtual DOM implementation used in siro.

mod element;
mod node;
mod text;
mod types;

pub use self::{
    element::{Attribute, Property, VElement},
    node::VNode,
    text::VText,
    types::{CowStr, FxIndexMap, FxIndexSet},
};
