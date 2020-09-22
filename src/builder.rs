pub mod html;
pub mod svg;

use crate::{
    callback::Callback,
    vdom::{Attribute, Element, Listener, Node, Property},
};
use gloo_events::EventListener;
use std::{borrow::Cow, rc::Rc};

pub trait ElementBuilder: Into<Node> {
    fn as_element_mut(&mut self) -> &mut Element;

    fn attribute(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<Attribute>) -> Self
    where
        Self: Sized,
    {
        self.as_element_mut()
            .attributes
            .insert(name.into(), value.into());
        self
    }

    fn property(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<Property>) -> Self
    where
        Self: Sized,
    {
        self.as_element_mut()
            .properties
            .insert(name.into(), value.into());
        self
    }

    fn listener(mut self, listener: Rc<dyn Listener>) -> Self
    where
        Self: Sized,
    {
        self.as_element_mut().listeners.replace(listener);
        self
    }

    fn child(mut self, child: impl Into<Node>) -> Self {
        self.as_element_mut().children.push(child.into());
        self
    }

    fn children(mut self, children: impl Children) -> Self {
        children.assign(&mut self.as_element_mut().children);
        self
    }

    fn class(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.as_element_mut().class_names.insert(value.into());
        self
    }

    fn id(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("id", value.into())
    }

    fn on(self, event_type: &'static str, callback: Callback<web::Event>) -> Self {
        struct CallbackListener {
            event_type: &'static str,
            callback: Callback<web::Event>,
        }

        impl Listener for CallbackListener {
            fn event_type(&self) -> &str {
                self.event_type
            }

            fn attach(self: Rc<Self>, target: &web::EventTarget) -> EventListener {
                EventListener::new(target, self.event_type, move |e| {
                    self.callback.invoke(e.clone());
                })
            }
        }

        self.listener(Rc::new(CallbackListener {
            event_type,
            callback,
        }))
    }
}

impl ElementBuilder for Element {
    fn as_element_mut(&mut self) -> &mut Element {
        self
    }
}

pub trait Children {
    fn assign(self, children: &mut Vec<Node>)
    where
        Self: Sized;
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl< $H, $($T),+ > Children for ( $H, $($T),+ )
        where
            $H: Into<Node>,
            $( $T: Into<Node>, )+
        {
            fn assign(self, children: &mut Vec<Node>) {
                #[allow(non_snake_case)]
                let ( $H, $($T),+ ) = self;

                children.push($H.into());
                $( children.push($T.into()); )+
            }
        }

        impl_children_for_tuples!( $($T),+ );
    };

    ( $C:ident ) => {
        impl< $C > Children for ( $C, )
        where
            $C: Into<Node>,
        {
            fn assign(self, children: &mut Vec<Node>) {
                children.push(self.0.into());
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);
