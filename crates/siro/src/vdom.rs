//! A virtual DOM implementation in `siro`.

mod element;
mod map;
mod text;

pub use element::{element, Attribute, Element, Property};
pub use map::Map;
pub use text::{text, Text};

use gloo_events::EventListener;
use std::{any::TypeId, borrow::Cow, fmt};
use wasm_bindgen::JsValue;

pub type CowStr = Cow<'static, str>;

/// A virtual DOM node.
pub trait Node {
    /// The message type associated with this node.
    type Msg: 'static;

    /// The cache for this node stored until the next rendering.
    type Cache: NodeCache;

    /// Render the node and obtain the cache value.
    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<Self::Cache, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>;

    /// Calculate diff with the previous rendering and apply its result.
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, cache: &mut Self::Cache) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>;

    /// Map the message type of this node to another one.
    fn map<F, TMsg: 'static>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
    {
        Map { node: self, f }
    }
}

/// The context for rendering virtual nodes.
pub trait Context {
    /// The message type associated with this context.
    type Msg: 'static;

    /// Create an `Element` node.
    fn create_element(
        &mut self,
        tag_name: &str,
        namespace_uri: Option<&str>,
    ) -> Result<web::Element, JsValue>;

    /// Create a `Text` node.
    fn create_text_node(&mut self, data: &str) -> Result<web::Text, JsValue>;

    /// Attach an event listener to the specified target.
    fn set_listener<F>(
        &mut self,
        target: &web::EventTarget,
        event_type: &'static str,
        callback: F,
    ) -> EventListener
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static;
}

/// The cached values for rendering virtual nodes.
pub trait NodeCache: AsRef<web::Node> + 'static {
    #[doc(hidden)] // private API
    fn __private_type_id__(&self) -> TypeId;
}

impl<T> NodeCache for T
where
    T: AsRef<web::Node> + 'static,
{
    fn __private_type_id__(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl fmt::Debug for dyn NodeCache + '_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("dyn_NodeCache").finish()
    }
}

impl dyn NodeCache + '_ {
    #[inline]
    fn is<T: NodeCache>(&self) -> bool {
        TypeId::of::<T>() == self.__private_type_id__()
    }

    #[inline]
    fn downcast_mut<T: NodeCache>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn NodeCache as *mut T)) }
        } else {
            None
        }
    }
}

pub(crate) fn diff<TNode, Ctx: ?Sized>(
    node: TNode,
    ctx: &mut Ctx,
    cache: &mut Box<dyn NodeCache>,
) -> Result<(), JsValue>
where
    TNode: Node,
    Ctx: Context<Msg = TNode::Msg>,
{
    if let Some(old) = cache.downcast_mut::<TNode::Cache>() {
        Node::diff(node, ctx, old)?;
    } else {
        let new = Node::render(node, ctx)?;
        crate::util::replace_node((&**cache).as_ref(), new.as_ref())?;
        *cache = Box::new(new);
    }
    Ok(())
}
