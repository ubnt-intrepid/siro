use super::{
    element::{Attribute, Element, Property},
    node::{Key, Node},
};
use gloo_events::EventListener;
use itertools::{EitherOrBoth, Itertools as _};
use rustc_hash::FxHashMap;
use wasm_bindgen::{prelude::*, JsCast as _};

fn set_attribute(element: &web::Element, name: &str, value: &Attribute) -> Result<(), JsValue> {
    match value {
        Attribute::String(value) => element.set_attribute(name, value)?,
        Attribute::Bool(true) => element.set_attribute(name, "")?,
        Attribute::Bool(false) => element.remove_attribute(name)?,
    }
    Ok(())
}

fn set_property(
    element: &web::Element,
    name: &str,
    value: Option<Property>,
) -> Result<(), JsValue> {
    #[allow(unused_unsafe)] // workaround(rust-analyzer)
    unsafe {
        js_sys::Reflect::set(element, &JsValue::from_str(name), &value.into())?;
    }
    Ok(())
}

#[derive(Default)]
pub(crate) struct Renderer {
    cached_nodes: FxHashMap<Key, web::Node>,
    cached_listeners: Vec<EventListener>,
}

impl Renderer {
    pub(crate) fn render(
        &mut self,
        node: &Node,
        document: &web::Document,
    ) -> Result<web::Node, JsValue> {
        let dom: web::Node = match node {
            Node::Element(e) => self.render_element(e, document)?.into(),
            Node::Text(t) => document.create_text_node(&*t.value).into(),
        };
        self.cached_nodes.insert(node.key(), dom.clone());
        Ok(dom)
    }

    fn render_element(
        &mut self,
        e: &Element,
        document: &web::Document,
    ) -> Result<web::Element, JsValue> {
        let name = wasm_bindgen::intern(e.tag_name);
        let element = match e.namespace_uri {
            Some(uri) => document.create_element_ns(Some(uri), name)?,
            None => document.create_element(name)?,
        };

        for (name, value) in &e.attributes {
            set_attribute(&element, name, value)?;
        }

        for (name, value) in &e.properties {
            set_property(element.as_ref(), &*name, Some(value.clone()))?;
        }

        for listener in &e.listeners {
            let listener = listener.clone().attach(element.as_ref());
            self.cached_listeners.push(listener);
        }

        for child in &e.children {
            let child_element = self.render(child, document)?;
            element.append_child(&child_element)?;
        }

        Ok(element)
    }

    pub(crate) fn diff(
        &mut self,
        old: &Node,
        new: &Node,
        document: &web::Document,
    ) -> Result<(), JsValue> {
        let old_key = old.key();
        let new_key = new.key();

        if old_key == new_key {
            // Same nodes.
            return Ok(());
        }

        let node = self.replant_cache_node(&old_key, &new_key);

        // FIXME: more efficient
        for listener in self.cached_listeners.drain(..) {
            drop(listener);
        }

        match (old, new) {
            (Node::Element(old), Node::Element(new)) if old.tag_name == new.tag_name => {
                let node = node
                    .dyn_ref::<web::Element>()
                    .expect_throw("cached node is not Element");
                self.diff_element(old, new, &node, document)?;
            }

            (Node::Text(current), Node::Text(new)) => {
                let node = node
                    .dyn_ref::<web::Text>()
                    .expect_throw("cache is not Text");
                if current.value != new.value {
                    node.set_data(&*new.value);
                }
            }

            (_, new) => {
                if let Some(parent) = node.parent_node() {
                    let replacement = self.render(new, document)?;
                    parent.replace_child(&replacement, &node)?;
                }
            }
        }

        Ok(())
    }

    fn replant_cache_node(&mut self, old: &Key, new: &Key) -> web::Node {
        let node = self
            .cached_nodes
            .remove(&old)
            .expect_throw("cache does not exist");
        self.cached_nodes.insert(new.clone(), node.clone());
        node
    }

    fn diff_element(
        &mut self,
        old: &Element,
        new: &Element,
        node: &web::Element,
        document: &web::Document,
    ) -> Result<(), JsValue> {
        for (name, new_value) in &new.attributes {
            match old.attributes.get(name) {
                Some(old) if old == new_value => (),
                _ => set_attribute(&node, name, new_value)?,
            }
        }

        for name in old.attributes.keys() {
            if !new.attributes.contains_key(name) {
                node.remove_attribute(name)?;
            }
        }

        for (name, new_value) in &new.properties {
            match old.properties.get(name) {
                Some(old) if old == new_value => (),
                _ => {
                    set_property(node.as_ref(), &*name, Some(new_value.clone()))?;
                }
            }
        }

        for name in old.properties.keys() {
            if !new.properties.contains_key(name) {
                set_property(node.as_ref(), &*name, None)?;
            }
        }

        // FIXME: more efficient
        for listener in &new.listeners {
            self.cached_listeners
                .push(listener.clone().attach(node.as_ref()));
        }

        for e in zip_longest(&old.children, &new.children) {
            match e {
                EitherOrBoth::Left(old) => {
                    let current = self
                        .cached_nodes
                        .remove(&old.key())
                        .expect_throw("cache does not exist");
                    node.remove_child(&current)?;
                }
                EitherOrBoth::Right(new) => {
                    let to_append = self.render(new, document)?;
                    node.append_child(&to_append)?;
                }
                EitherOrBoth::Both(old, new) => {
                    self.diff(old, new, document)?;
                }
            }
        }

        Ok(())
    }
}

#[inline]
fn zip_longest<I, J>(i: I, j: J) -> itertools::ZipLongest<I::IntoIter, J::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator,
{
    i.into_iter().zip_longest(j)
}
