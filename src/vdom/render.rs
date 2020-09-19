use super::{
    cache::CachedNodes,
    element::{Attribute, Property},
    node::Node,
};
use gloo_events::EventListener;
use itertools::{EitherOrBoth, Itertools as _};
use wasm_bindgen::{prelude::*, JsCast as _};
use web_sys as web;

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

pub fn render(
    node: &Node,
    document: &web::Document,
    caches: &mut CachedNodes,
    event_listeners: &mut Vec<EventListener>,
) -> Result<web::Node, JsValue> {
    match node {
        Node::Element(e) => {
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

            for child in &e.children {
                let child_element = render(child, document, caches, event_listeners)?;
                element.append_child(&child_element)?;
            }

            for listener in &e.listeners {
                event_listeners.push(listener.clone().attach(element.as_ref()));
            }

            let node: web::Node = element.into();
            caches.set(e.key(), node.clone());
            Ok(node)
        }

        Node::Text(t) => {
            let node: web::Node = document.create_text_node(&*t.value).into();
            caches.set(t.key(), node.clone());
            Ok(node)
        }
    }
}

pub fn diff(
    old: &Node,
    new: &Node,
    document: &web::Document,
    caches: &mut CachedNodes,
    event_listeners: &mut Vec<EventListener>,
) -> Result<(), JsValue> {
    if old.key() == new.key() {
        // Same nodes.
        return Ok(());
    }

    let node = caches
        .remove(old.key())
        .expect_throw("cache does not exist");
    caches.set(new.key(), node.clone());

    match (old, new) {
        (Node::Element(old), Node::Element(new)) if old.tag_name == new.tag_name => {
            let node = node
                .dyn_ref::<web::Element>()
                .expect_throw("cached node is not Element");

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
                event_listeners.push(listener.clone().attach(node.as_ref()));
            }

            for e in zip_longest(&old.children, &new.children) {
                match e {
                    EitherOrBoth::Left(old) => {
                        let current = caches
                            .remove(old.key())
                            .expect_throw("cache does not exist");
                        node.remove_child(&current)?;
                    }
                    EitherOrBoth::Right(new) => {
                        let to_append = render(new, document, caches, event_listeners)?;
                        node.append_child(&to_append)?;
                    }
                    EitherOrBoth::Both(old, new) => {
                        diff(old, new, document, caches, event_listeners)?;
                    }
                }
            }
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
                let replacement = render(new, document, caches, event_listeners)?;
                parent.replace_child(&replacement, &node)?;
            }
        }
    }

    Ok(())
}

#[inline]
fn zip_longest<I, J>(i: I, j: J) -> itertools::ZipLongest<I::IntoIter, J::IntoIter>
where
    I: IntoIterator,
    J: IntoIterator,
{
    i.into_iter().zip_longest(j)
}
