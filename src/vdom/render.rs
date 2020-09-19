use super::{cache::CachedNodes, node::Node};
use gloo_events::EventListener;
use itertools::{EitherOrBoth, Itertools as _};
use wasm_bindgen::{prelude::*, JsCast as _};
use web_sys as web;

pub fn render(
    node: &Node,
    document: &web::Document,
    caches: &mut CachedNodes,
    event_listeners: &mut Vec<EventListener>,
) -> Result<web::Node, JsValue> {
    match node {
        Node::Element(e) => {
            let element = document.create_element(wasm_bindgen::intern(&*e.tag_name))?;

            for (name, value) in &e.attrs {
                element.set_attribute(name, value)?;
            }

            for (name, value) in &e.properties {
                let value = JsValue::from(value.clone());
                js_sys::Reflect::set(element.as_ref(), &name.into(), &value)?;
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

            for (name, new_value) in &new.attrs {
                match old.attrs.get(name) {
                    Some(old) if old == new_value => (),
                    _ => {
                        node.set_attribute(name, new_value)?;
                    }
                }
            }

            for name in old.attrs.keys() {
                if !new.attrs.contains_key(name) {
                    node.remove_attribute(name)?;
                }
            }

            for (name, new_value) in &new.properties {
                match old.properties.get(name) {
                    Some(old) if old == new_value => (),
                    _ => {
                        let new_value = JsValue::from(new_value.clone());
                        js_sys::Reflect::set(node.as_ref(), &name.into(), &new_value)?;
                    }
                }
            }

            for name in old.properties.keys() {
                if !new.properties.contains_key(name) {
                    js_sys::Reflect::set(node.as_ref(), &name.into(), &JsValue::UNDEFINED)?;
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
