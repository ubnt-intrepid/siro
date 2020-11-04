use crate::app::App;
use crate::subscription::Subscription;
use wasm_bindgen::prelude::*;

pub struct Env {
    pub(crate) window: web::Window,
    pub(crate) document: web::Document,
}

impl Env {
    pub fn new() -> Result<Self, JsValue> {
        let window = web::window().ok_or("no global Window exists")?;
        let document = window.document().ok_or("no Document exists")?;
        Ok(Self { window, document })
    }

    pub fn window(&self) -> &web::Window {
        &self.window
    }

    pub fn mount<TMsg>(&self, selector: &str) -> Result<App<TMsg>, JsValue>
    where
        TMsg: 'static,
    {
        let node = self
            .document
            .query_selector(selector)?
            .ok_or("missing node")?;
        Ok(App::new(self, node.into()))
    }

    pub fn mount_to_body<TMsg>(&self) -> Result<App<TMsg>, JsValue>
    where
        TMsg: 'static,
    {
        let body = self.document.body().ok_or("missing body in document")?;
        Ok(App::new(self, body.into()))
    }

    pub fn subscribe<S>(&self, subscription: S) -> Result<S::Stream, JsValue>
    where
        S: Subscription,
    {
        subscription.subscribe(self)
    }
}
