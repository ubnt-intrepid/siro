mod element;
mod map;
mod text;

pub use element::{element, Element};
pub use map::Map;
pub use text::{text, Text};

use crate::vdom::{CowStr, VElement, VNode, VText};
use gloo_events::EventListener;
use wasm_bindgen::JsValue;

pub trait Context {
    type Msg: 'static;

    fn create_element(
        &mut self,
        tag_name: CowStr,
        namespace_uri: Option<CowStr>,
    ) -> Result<VElement, JsValue>;

    fn create_text_node(&mut self, data: CowStr) -> Result<VText, JsValue>;

    fn create_listener<F>(
        &mut self,
        target: &web::EventTarget,
        event_type: &'static str,
        callback: F,
    ) -> EventListener
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static;
}

/// The view object that renders virtual DOM.
pub trait View {
    /// The message type associated with this view.
    type Msg: 'static;

    /// Render the virtual DOM.
    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>;

    /// Calculate diff with the old `VNode`.
    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, old: &mut VNode) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>;
}

pub trait ViewExt: View {
    fn map<F, TMsg: 'static>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
    {
        Map { view: self, f }
    }
}

impl<TView> ViewExt for TView where TView: View {}
