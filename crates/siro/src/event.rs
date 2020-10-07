use crate::{
    attr::Attr,
    mailbox::{Mailbox, Sender},
    vdom::{Listener, VElement},
};
use gloo_events::EventListener;
use std::rc::Rc;

pub fn on_event<F, TMsg>(event_type: &'static str, f: F) -> OnEvent<F>
where
    F: for<'a> Fn(&'a web::Event) -> Option<TMsg> + Clone + 'static,
    TMsg: 'static,
{
    OnEvent { event_type, f }
}

pub struct OnEvent<F> {
    event_type: &'static str,
    f: F,
}

impl<F, TMsg> Attr<TMsg> for OnEvent<F>
where
    F: Fn(&web::Event) -> Option<TMsg> + Clone + 'static,
    TMsg: 'static,
{
    fn apply<M: ?Sized>(self, element: &mut VElement, mailbox: &M)
    where
        M: Mailbox<Msg = TMsg>,
    {
        element
            .listeners
            .replace(Box::new(OnEventListener(Rc::new(Inner {
                event_type: self.event_type,
                f: self.f,
                sender: mailbox.sender(),
            }))));
    }
}

struct OnEventListener<F, S>(Rc<Inner<F, S>>);

struct Inner<F, S> {
    event_type: &'static str,
    f: F,
    sender: S,
}

impl<F, S, TMsg> Listener for OnEventListener<F, S>
where
    F: Fn(&web::Event) -> Option<TMsg> + 'static,
    S: Sender<Msg = TMsg>,
    TMsg: 'static,
{
    fn event_type(&self) -> &'static str {
        self.0.event_type
    }

    fn attach(&self, target: &web::EventTarget) -> EventListener {
        let inner = self.0.clone();
        EventListener::new(target, self.event_type(), move |e| {
            if let Some(msg) = (inner.f)(e) {
                inner.sender.send_message(msg);
            }
        })
    }
}

pub fn on<F, TMsg>(
    event_type: &'static str,
    f: F,
) -> OnEvent<impl for<'a> Fn(&'a web::Event) -> Option<TMsg> + Clone + 'static>
where
    F: for<'a> Fn(&'a web::Event) -> TMsg + Clone + 'static,
    TMsg: 'static,
{
    on_event(event_type, move |event| Some(f(event)))
}

pub fn on_input<TMsg: 'static>(
    f: impl Fn(String) -> TMsg + Clone + 'static,
) -> OnEvent<impl for<'a> Fn(&'a web::Event) -> Option<TMsg> + Clone + 'static> {
    on_event("input", move |event| {
        let value = js_sys::Reflect::get(
            &&event.target()?, //
            &"value".into(),
        )
        .ok()?
        .as_string()?;
        Some(f(value))
    })
}
