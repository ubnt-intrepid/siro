use crate::app::App;
use crate::subscription::Subscription;

pub struct Env {
    pub(crate) window: web::Window,
    pub(crate) document: web::Document,
}

impl Env {
    pub fn new() -> crate::Result<Self> {
        let window =
            web::window().ok_or_else(|| crate::Error::custom("no global Window exists"))?;
        let document = window
            .document()
            .ok_or_else(|| crate::Error::custom("no Document exists"))?;
        Ok(Self { window, document })
    }

    pub fn window(&self) -> &web::Window {
        &self.window
    }

    pub fn mount<TMsg>(&self, selector: &str) -> crate::Result<App<TMsg>>
    where
        TMsg: 'static,
    {
        let node = self
            .document
            .query_selector(selector)
            .map_err(crate::Error::caught_from_js)?
            .ok_or_else(|| crate::Error::custom("missing node"))?;
        Ok(App::new(self, node.into()))
    }

    pub fn mount_to_body<TMsg>(&self) -> crate::Result<App<TMsg>>
    where
        TMsg: 'static,
    {
        let body = self
            .document
            .body()
            .ok_or_else(|| crate::Error::custom("missing body in document"))?;
        Ok(App::new(self, body.into()))
    }

    pub fn subscribe<S>(&self, subscription: S) -> crate::Result<S::Stream>
    where
        S: Subscription,
    {
        subscription.subscribe(self)
    }
}
