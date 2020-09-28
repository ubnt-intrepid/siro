mod element;
mod node;
mod render;
mod text;

pub use self::{
    element::{Attribute, Listener, Property, VElement},
    node::{CustomNode, VNode},
    text::VText,
};

pub(crate) use self::render::Renderer;

type BuildFxHasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;

/// A type alias of associate map used within this library.
pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;

/// A type alias of associate set used within this library.
pub type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;
