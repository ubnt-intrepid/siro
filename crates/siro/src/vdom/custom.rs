use super::id::{NodeId, NodeIdAnchor};
use std::fmt;
use wasm_bindgen::JsValue;

pub struct CustomNode {
    anchor: NodeIdAnchor,
    render: Box<dyn Fn(&web::Document) -> Result<web::Node, JsValue>>,
}

impl fmt::Debug for CustomNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomNode").finish()
    }
}

impl CustomNode {
    pub fn new<F>(render: F) -> Self
    where
        F: Fn(&web::Document) -> Result<web::Node, JsValue> + 'static,
    {
        Self {
            anchor: NodeIdAnchor::default(),
            render: Box::new(render),
        }
    }

    pub(crate) fn id(&self) -> NodeId {
        self.anchor.id()
    }

    pub(crate) fn render(&self, document: &web::Document) -> Result<web::Node, JsValue> {
        (self.render)(document)
    }
}
