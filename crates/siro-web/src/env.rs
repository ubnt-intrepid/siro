use crate::app::App;
use crate::subscription::Subscription;

pub struct Env {
    pub(crate) window: web::Window,
    pub(crate) document: web::Document,
    pub(crate) local_storage: Option<web::Storage>,
}

impl Env {
    pub fn new() -> crate::Result<Self> {
        let window =
            web::window().ok_or_else(|| crate::Error::custom("no global Window exists"))?;

        let document = window
            .document()
            .ok_or_else(|| crate::Error::custom("no Document exists"))?;

        let local_storage = window
            .local_storage()
            .map_err(crate::Error::caught_from_js)?;

        Ok(Self {
            window,
            document,
            local_storage,
        })
    }

    pub fn current_url_hash(&self) -> Option<String> {
        self.window.location().hash().ok()
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

    pub fn get_storage_item(&self, key: &str) -> crate::Result<Option<String>> {
        match self.local_storage {
            Some(ref storage) => storage.get_item(key).map_err(crate::Error::caught_from_js),
            None => Ok(None),
        }
    }

    pub fn set_storage_item(&self, key: &str, value: String) -> crate::Result<()> {
        match self.local_storage {
            Some(ref storage) => storage
                .set_item(key, &*value)
                .map_err(crate::Error::caught_from_js),
            None => Ok(()),
        }
    }
}
