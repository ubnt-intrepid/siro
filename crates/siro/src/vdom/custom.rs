use super::node::Id;
use std::{fmt, rc::Rc};
use wasm_bindgen::JsValue;

pub struct CustomNode {
    rc: Rc<()>,
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
            rc: Rc::new(()),
            render: Box::new(render),
        }
    }

    pub(super) fn id(&self) -> Id {
        Id::new(&self.rc)
    }

    pub(super) fn render(&self, document: &web::Document) -> Result<web::Node, JsValue> {
        (self.render)(document)
    }
}
