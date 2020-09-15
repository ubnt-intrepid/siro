use crate::Meow;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys as web;

pub enum Node {
    Text(Text),
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Self::Text(text)
    }
}

impl From<&'static str> for Node {
    fn from(text: &'static str) -> Self {
        Self::from(Text::new(text))
    }
}

impl From<String> for Node {
    fn from(text: String) -> Self {
        Self::from(Text::new(text))
    }
}

impl From<Cow<'static, str>> for Node {
    fn from(text: Cow<'static, str>) -> Self {
        Self::from(Text::new(text))
    }
}

impl Node {
    pub fn render(&mut self, meow: &Meow) -> web::Node {
        match self {
            Node::Text(t) => t.render(meow),
        }
    }

    pub fn diff(&mut self, meow: &Meow, new: Node) -> web::Node {
        match (self, new) {
            (Node::Text(current), Node::Text(new)) => current.diff(meow, new),
        }
    }
}

// ==== Text ====

pub struct Text {
    value: Cow<'static, str>,
    text_node: Option<web::Text>,
}

impl Text {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            text_node: None,
        }
    }

    pub(crate) fn render(&mut self, meow: &Meow) -> web::Node {
        let text_node = meow.document.create_text_node(&*self.value);
        self.text_node.replace(text_node.clone());
        text_node.into()
    }

    pub(crate) fn diff(&mut self, _meow: &Meow, new: Text) -> web::Node {
        let text_node = self.text_node.as_ref().unwrap_throw();
        if self.value != new.value {
            let _ = std::mem::replace(&mut self.value, new.value);
            text_node.set_data(&self.value);
        }
        text_node.clone().into()
    }
}
