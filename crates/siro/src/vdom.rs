//! A virtual DOM implementation used in siro.

mod custom;
mod element;
mod id;
mod node;
mod text;
mod types;

pub use self::{
    custom::CustomNode,
    element::{Attribute, Listener, Property, VElement},
    node::VNode,
    text::VText,
    types::{CowStr, FxIndexMap, FxIndexSet},
};

pub(crate) use self::id::NodeId;
