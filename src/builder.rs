use crate::{
    event::{EventHandler, EventHandlerBase},
    mailbox::{Mailbox, Sender},
    vdom::{Attribute, Listener, Property, VElement, VNode},
};
use gloo_events::EventListener;
use std::{borrow::Cow, rc::Rc};

pub trait Element: Into<VNode> {
    fn as_velement(&mut self) -> &mut VElement;

    fn attribute(
        mut self,
        name: impl Into<Cow<'static, str>>,
        value: impl Into<Attribute>,
    ) -> Self {
        self.as_velement()
            .attributes
            .insert(name.into(), value.into());
        self
    }

    fn property(mut self, name: impl Into<Cow<'static, str>>, value: impl Into<Property>) -> Self {
        self.as_velement()
            .properties
            .insert(name.into(), value.into());
        self
    }

    fn listener(mut self, listener: Rc<dyn Listener>) -> Self {
        self.as_velement().listeners.replace(listener);
        self
    }

    fn child(mut self, child: impl Into<VNode>) -> Self {
        self.as_velement().children.push(child.into());
        self
    }

    fn children(mut self, children: impl Children) -> Self {
        children.assign(&mut self.as_velement().children);
        self
    }

    fn append(self, iter: impl IntoIterator<Item = impl Into<VNode>>) -> Self {
        struct IterChildren<I>(I);

        impl<I> Children for IterChildren<I>
        where
            I: IntoIterator,
            I::Item: Into<VNode>,
        {
            fn assign(self, children: &mut Vec<VNode>) {
                children.extend(self.0.into_iter().map(Into::into));
            }
        }

        self.children(IterChildren(iter))
    }

    fn class(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.as_velement().class_names.insert(value.into());
        self
    }

    fn id(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.attribute("id", value.into())
    }

    fn event<M, E>(self, mailbox: M, handler: E) -> Self
    where
        M: Mailbox,
        E: EventHandler<Self, Msg = M::Msg> + 'static,
    {
        struct EventHandlerListener<S, E> {
            sender: S,
            handler: E,
        }

        impl<S, E> Listener for EventHandlerListener<S, E>
        where
            S: Sender + 'static,
            E: EventHandlerBase<Msg = S::Msg> + 'static,
        {
            fn event_type(&self) -> &str {
                self.handler.event_type()
            }

            fn attach(self: Rc<Self>, target: &web::EventTarget) -> EventListener {
                EventListener::new(target, self.handler.event_type(), move |e| {
                    if let Some(msg) = self.handler.invoke(e) {
                        self.sender.send_message(msg);
                    }
                })
            }
        }

        self.listener(Rc::new(EventHandlerListener {
            sender: mailbox.sender(),
            handler,
        }))
    }
}

impl Element for VElement {
    fn as_velement(&mut self) -> &mut VElement {
        self
    }
}

pub trait Children {
    fn assign(self, children: &mut Vec<VNode>)
    where
        Self: Sized;
}

macro_rules! impl_children_for_tuples {
    ( $H:ident, $($T:ident),+ ) => {
        impl< $H, $($T),+ > Children for ( $H, $($T),+ )
        where
            $H: Into<VNode>,
            $( $T: Into<VNode>, )+
        {
            fn assign(self, children: &mut Vec<VNode>) {
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
            $C: Into<VNode>,
        {
            fn assign(self, children: &mut Vec<VNode>) {
                children.push(self.0.into());
            }
        }
    };
}

impl_children_for_tuples!(
    C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, //
    C11, C12, C13, C14, C15, C16, C17, C18, C19, C20
);
