use rustc_hash::FxHasher;
use std::{borrow::Cow, hash::BuildHasherDefault};

type BuildFxHasher = BuildHasherDefault<FxHasher>;

/// A type alias of associate map used in virtual nodes.
pub type FxIndexMap<K, V> = indexmap::IndexMap<K, V, BuildFxHasher>;

/// A type alias of associate set used in virtual nodes.
pub type FxIndexSet<T> = indexmap::IndexSet<T, BuildFxHasher>;

/// A type alias of Clone-on-write string.
pub type CowStr = Cow<'static, str>;
