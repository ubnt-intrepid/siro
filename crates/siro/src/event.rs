use crate::attr::{self, Attr};
use wasm_bindgen::JsValue;

pub fn on_event<TMsg: 'static>(
    event_type: &'static str,
    f: impl Fn(&web::Event) -> Option<TMsg> + 'static,
) -> impl Attr<TMsg> {
    OnEvent { event_type, f }
}

struct OnEvent<F> {
    event_type: &'static str,
    f: F,
}

impl<F, TMsg> Attr<TMsg> for OnEvent<F>
where
    F: Fn(&web::Event) -> Option<TMsg> + 'static,
    TMsg: 'static,
{
    fn apply<Ctx: ?Sized>(self, ctx: &mut Ctx) -> Result<(), JsValue>
    where
        Ctx: attr::Context<Msg = TMsg>,
    {
        ctx.set_listener(self.event_type, self.f)?;
        Ok(())
    }
}

pub fn on<TMsg: 'static>(
    event_type: &'static str,
    f: impl Fn(&web::Event) -> TMsg + Clone + 'static,
) -> impl Attr<TMsg> {
    on_event(event_type, move |event| Some(f(event)))
}

// ==== common event handlers ====

pub fn on_input<TMsg: 'static>(f: impl Fn(String) -> TMsg + Clone + 'static) -> impl Attr<TMsg> {
    on_event("input", move |e| {
        let value = js_sys::Reflect::get(&&e.target()?, &JsValue::from_str("value"))
            .ok()?
            .as_string()?;
        Some(f(value))
    })
}

pub fn on_enter<TMsg: 'static>(f: impl Fn() -> TMsg + Clone + 'static) -> impl Attr<TMsg> {
    on_event("keydown", move |e: &web::Event| {
        let key = js_sys::Reflect::get(e.as_ref(), &JsValue::from_str("key"))
            .ok()?
            .as_string()?;
        match &*key {
            "Enter" => Some(f()),
            _ => None,
        }
    })
}
