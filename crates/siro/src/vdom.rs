//! A virtual DOM implementation in `siro`.

mod element;
mod map;
mod text;

pub use element::{element, Attribute, Element, Property};
pub use map::Map;
pub use text::{text, Text};

use gloo_events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::JsValue;

use element::VElement;
use text::VText;

pub type CowStr = Cow<'static, str>;

/// A virtual DOM node.
pub trait Node {
    /// The message type associated with this node.
    type Msg: 'static;

    /// Render the node.
    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>;

    /// Calculate diff with the old `VNode`.
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, old: &mut VNode) -> Result<(), JsValue>
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

/// The cached values of rendered virtual node.
#[derive(Debug)]
#[non_exhaustive]
pub enum VNode {
    Element(VElement),
    Text(VText),
}

impl AsRef<web::Node> for VNode {
    fn as_ref(&self) -> &web::Node {
        match self {
            VNode::Element(e) => e.as_ref(),
            VNode::Text(t) => t.as_ref(),
        }
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
