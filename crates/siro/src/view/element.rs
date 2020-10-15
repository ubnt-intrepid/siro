use super::{Context, View};
use crate::{
    attr::{self, Attr},
    children::{self, Children},
    vdom::{Attribute, CowStr, FxIndexMap, FxIndexSet, Property, VElement, VNode},
};
use gloo_events::EventListener;
use std::marker::PhantomData;
use wasm_bindgen::JsValue;

pub fn element<TMsg: 'static, A, C>(
    tag_name: impl Into<CowStr>,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
) -> Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Children<TMsg>,
{
    Element {
        tag_name: tag_name.into(),
        namespace_uri,
        attr,
        children,
        _marker: PhantomData,
    }
}

pub struct Element<TMsg, A, C> {
    tag_name: CowStr,
    namespace_uri: Option<CowStr>,
    attr: A,
    children: C,
    _marker: PhantomData<fn() -> TMsg>,
}

impl<TMsg: 'static, A, C> View for Element<TMsg, A, C>
where
    A: Attr<TMsg>,
    C: Children<TMsg>,
{
    type Msg = TMsg;

    fn render<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<VNode, JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        let mut velement = ctx.create_element(self.tag_name, self.namespace_uri)?;

        let node = velement.node.clone();
        let classes = node.class_list();
        let style = js_sys::Reflect::get(&node, &JsValue::from_str("style"))?;

        self.attr.apply(&mut NewAttrs {
            ctx,
            velement: &mut velement,
            node: &node,
            classes: &classes,
            style: &style,
        })?;

        if let None = velement.inner_html {
            let mut cursor = 0;
            self.children.diff(&mut AppendChildren {
                ctx,
                vnodes: &mut velement.children,
                cursor: &mut cursor,
                parent: &node,
            })?;
        }

        Ok(velement.into())
    }

    fn diff<Ctx: ?Sized>(self, ctx: &mut Ctx, old: &mut VNode) -> Result<(), JsValue>
    where
        Ctx: Context<Msg = Self::Msg>,
    {
        match old {
            VNode::Element(element)
                if element.tag_name == self.tag_name
                    && element.namespace_uri == self.namespace_uri =>
            {
                let classes = element.node.class_list();
                let style = js_sys::Reflect::get(&element.node, &JsValue::from_str("style"))?;

                {
                    let mut cx = DiffAttrs::new(&mut *ctx, element, &classes, &style);
                    self.attr.apply(&mut cx)?;
                    cx.finish()?;
                }

                if let None = element.inner_html {
                    let mut cursor = 0;
                    self.children.diff(&mut AppendChildren {
                        ctx,
                        vnodes: &mut element.children,
                        cursor: &mut cursor,
                        parent: &element.node,
                    })?;
                    for child in element.children.drain(cursor..) {
                        element.node.remove_child(child.as_node())?;
                    }
                }
            }

            _ => {
                let new = View::render(self, ctx)?;
                crate::util::replace_node(old.as_node(), new.as_node())?;
                *old = new;
            }
        }
        Ok(())
    }
}

// === context types ====

struct NewAttrs<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
    velement: &'a mut VElement,
    node: &'a web::Element,
    classes: &'a web::DomTokenList,
    style: &'a JsValue,
}

impl<Ctx: ?Sized> attr::Context for NewAttrs<'_, Ctx>
where
    Ctx: Context,
{
    type Msg = Ctx::Msg;

    fn set_attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), JsValue> {
        set_attribute(&self.node, &*name, &value)?;
        self.velement.attributes.insert(name, value);
        Ok(())
    }

    fn set_property(&mut self, name: CowStr, value: Property) -> Result<(), JsValue> {
        set_property(&self.node, &*name, Some(value.clone()))?;
        self.velement.properties.insert(name, value);
        Ok(())
    }

    fn set_listener<F>(&mut self, event_type: &'static str, callback: F) -> Result<(), JsValue>
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static,
    {
        let listener = self
            .ctx
            .create_listener(self.node.as_ref(), event_type, callback);
        self.velement.listeners.insert(event_type.into(), listener);
        Ok(())
    }

    fn add_class(&mut self, name: CowStr) -> Result<(), JsValue> {
        self.classes.add_1(&*name)?;
        self.velement.classes.replace(name);
        Ok(())
    }

    fn add_style(&mut self, name: CowStr, value: CowStr) -> Result<(), JsValue> {
        js_sys::Reflect::set(
            &self.style,
            &JsValue::from_str(&*name),
            &JsValue::from_str(&*value),
        )?;
        self.velement.styles.insert(name, value);
        Ok(())
    }

    fn set_inner_html(&mut self, inner_html: CowStr) -> Result<(), JsValue> {
        self.node.set_inner_html(&*inner_html);
        self.velement.inner_html.replace(inner_html);
        Ok(())
    }
}

// FIXME: more efficient!

struct DiffAttrs<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
    old: &'a mut VElement,
    classes: &'a web::DomTokenList,
    style: &'a JsValue,
    new_attributes: FxIndexMap<CowStr, Attribute>,
    new_properties: FxIndexMap<CowStr, Property>,
    new_listeners: FxIndexMap<CowStr, EventListener>,
    new_classes: FxIndexSet<CowStr>,
    new_styles: FxIndexMap<CowStr, CowStr>,
}

impl<'a, Ctx: ?Sized> DiffAttrs<'a, Ctx>
where
    Ctx: Context,
{
    fn new(
        ctx: &'a mut Ctx,
        old: &'a mut VElement,
        classes: &'a web::DomTokenList,
        style: &'a JsValue,
    ) -> Self {
        Self {
            ctx,
            old,
            classes,
            style,
            new_attributes: FxIndexMap::default(),
            new_properties: FxIndexMap::default(),
            new_listeners: FxIndexMap::default(),
            new_classes: FxIndexSet::default(),
            new_styles: FxIndexMap::default(),
        }
    }

    fn finish(self) -> Result<(), JsValue> {
        let old_attributes = std::mem::replace(&mut self.old.attributes, self.new_attributes);
        for name in old_attributes.keys() {
            self.old.node.remove_attribute(name)?;
        }

        let old_properties = std::mem::replace(&mut self.old.properties, self.new_properties);
        for name in old_properties.keys() {
            set_property(&self.old.node, name, None)?;
        }

        let _ = std::mem::replace(&mut self.old.listeners, self.new_listeners);

        let old_classes = std::mem::replace(&mut self.old.classes, self.new_classes);
        for class in old_classes {
            self.classes.remove_1(&*class)?;
        }

        let old_styles = std::mem::replace(&mut self.old.styles, self.new_styles);
        for style in old_styles.keys() {
            js_sys::Reflect::set(&*self.style, &JsValue::from_str(style), &JsValue::UNDEFINED)?;
        }

        Ok(())
    }
}

impl<Ctx: ?Sized> attr::Context for DiffAttrs<'_, Ctx>
where
    Ctx: Context,
{
    type Msg = Ctx::Msg;

    fn set_attribute(&mut self, name: CowStr, value: Attribute) -> Result<(), JsValue> {
        match self.old.attributes.remove(&name) {
            Some(old_value) if old_value == value => (),
            _ => set_attribute(&self.old.node, &name, &value)?,
        }
        self.new_attributes.insert(name, value);
        Ok(())
    }

    fn set_property(&mut self, name: CowStr, value: Property) -> Result<(), JsValue> {
        match self.old.properties.remove(&name) {
            Some(old_value) if old_value == value => (),
            _ => set_property(&self.old.node, &name, Some(value.clone()))?,
        }
        self.new_properties.insert(name, value);
        Ok(())
    }

    fn set_listener<F>(&mut self, event_type: &'static str, callback: F) -> Result<(), JsValue>
    where
        F: Fn(&web::Event) -> Option<Self::Msg> + 'static,
    {
        let listener = self
            .ctx
            .create_listener(self.old.node.as_ref(), event_type, callback);
        self.new_listeners.insert(event_type.into(), listener);
        Ok(())
    }

    fn add_class(&mut self, name: CowStr) -> Result<(), JsValue> {
        if !self.old.classes.remove(&name) {
            self.classes.add_1(&name)?;
        }
        self.new_classes.replace(name);
        Ok(())
    }

    fn add_style(&mut self, name: CowStr, value: CowStr) -> Result<(), JsValue> {
        match self.old.styles.remove(&name) {
            Some(old_value) if old_value == value => (),
            _ => {
                js_sys::Reflect::set(
                    &self.style,
                    &JsValue::from_str(&name),
                    &JsValue::from_str(&value),
                )?;
            }
        }
        self.new_styles.insert(name, value);
        Ok(())
    }

    fn set_inner_html(&mut self, inner_html: CowStr) -> Result<(), JsValue> {
        self.old.node.set_inner_html(&*inner_html);
        self.old.inner_html = Some(inner_html);
        self.old.children.clear();
        Ok(())
    }
}

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
    js_sys::Reflect::set(element, &JsValue::from_str(name), &value.into())?;
    Ok(())
}

struct AppendChildren<'a, Ctx: ?Sized> {
    ctx: &'a mut Ctx,
    vnodes: &'a mut Vec<VNode>,
    cursor: &'a mut usize,
    parent: &'a web::Element,
}

impl<Ctx: ?Sized> children::Context for AppendChildren<'_, Ctx>
where
    Ctx: Context,
{
    type Msg = Ctx::Msg;

    fn append_child<TView>(&mut self, view: TView) -> Result<(), JsValue>
    where
        TView: View<Msg = Self::Msg>,
    {
        if let Some(old) = self.vnodes.get_mut(*self.cursor) {
            View::diff(view, &mut *self.ctx, old)?;
        } else {
            let vnode = View::render(view, &mut *self.ctx)?;
            self.parent.append_child(vnode.as_node())?;
            self.vnodes.push(vnode);
        }
        *self.cursor += 1;
        Ok(())
    }
}
