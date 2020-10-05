use crate::{
    mailbox::{Mailbox, Sender},
    vdom::{Listener, VNode},
    view::{ModifyView, View},
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

impl<TView, F, TMsg> ModifyView<TView> for OnEvent<F>
where
    TView: View<Msg = TMsg>,
    F: Fn(&web::Event) -> Option<TMsg> + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;
    type View = OnEventView<TView, F>;

    fn modify(self, view: TView) -> Self::View {
        OnEventView {
            view,
            modifier: self,
        }
    }
}

pub struct OnEventView<TView, F> {
    view: TView,
    modifier: OnEvent<F>,
}

impl<TView, F, TMsg> View for OnEventView<TView, F>
where
    TView: View<Msg = TMsg>,
    F: Fn(&web::Event) -> Option<TMsg> + Clone + 'static,
    TMsg: 'static,
{
    type Msg = TMsg;

    fn render<M: ?Sized>(self, mailbox: &M) -> VNode
    where
        M: Mailbox<Msg = Self::Msg>,
    {
        match self.view.render(mailbox) {
            VNode::Element(mut element) => {
                element
                    .listeners
                    .replace(Box::new(OnEventListener(Rc::new(Inner {
                        event_type: self.modifier.event_type,
                        f: self.modifier.f.clone(),
                        sender: mailbox.sender(),
                    }))));
                element.into()
            }
            node => node,
        }
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
