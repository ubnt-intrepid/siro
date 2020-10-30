use crate::render::{RenderContext, VNode};
use futures::{
    channel::mpsc, //
    future::LocalBoxFuture,
    prelude::*,
    select,
    stream::FuturesUnordered,
};
use serde::Deserialize;
use siro::{
    subscription::{Mailbox, Subscriber, Subscription},
    vdom::Nodes,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use wasm_bindgen_futures::JsFuture;

pub struct App<TMsg: 'static> {
    mountpoint: web::Node,
    window: web::Window,
    document: web::Document,
    vnodes: Vec<VNode>,
    tx: mpsc::UnboundedSender<TMsg>,
    rx: mpsc::UnboundedReceiver<TMsg>,
    cmd_tasks: FuturesUnordered<LocalBoxFuture<'static, TMsg>>,
}

impl<TMsg: 'static> App<TMsg> {
    fn new(mountpoint: web::Node, window: web::Window, document: web::Document) -> Self {
        let (tx, rx) = mpsc::unbounded();
        Self {
            mountpoint,
            window,
            document,
            vnodes: vec![],
            tx,
            rx,
            cmd_tasks: FuturesUnordered::new(),
        }
    }

    pub fn mount(selector: &str) -> Result<Self, JsValue> {
        let window = web::window().ok_or("no global Window exists")?;
        let document = window.document().ok_or("no Document exists")?;
        let mountpoint = document.query_selector(selector)?.ok_or("missing node")?;
        Ok(Self::new(mountpoint.into(), window, document))
    }

    pub fn mount_to_body() -> Result<Self, JsValue> {
        let window = web::window().ok_or("no global Window exists")?;
        let document = window.document().ok_or("no Document exists")?;
        let body = document.body().ok_or("missing body in document")?;
        Ok(Self::new(body.into(), window, document))
    }

    /// Register a `Subscription`.
    pub fn subscribe<S>(&self, subscription: S) -> Result<S::Subscribe, S::Error>
    where
        S: Subscription<Msg = TMsg>,
    {
        subscription.subscribe(AppSubscriber { tx: &self.tx })
    }

    pub fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }

    pub fn start_fetch<F, T>(&mut self, url: String, f: F)
    where
        F: FnOnce(Result<T, String>) -> TMsg + 'static,
        T: for<'de> Deserialize<'de>,
    {
        let window = self.window.clone();
        self.cmd_tasks.push(Box::pin(async move {
            let response = do_fetch(&window, &url).await;
            f(response)
        }));
    }

    pub async fn next_message(&mut self) -> Option<TMsg> {
        select! {
            msg = self.rx.select_next_some() => Some(msg),
            msg = self.cmd_tasks.select_next_some() => Some(msg),
            complete => None,
        }
    }

    pub fn render<N>(&mut self, nodes: N) -> Result<(), JsValue>
    where
        N: Nodes<TMsg>,
    {
        RenderContext {
            document: &self.document,
            parent: &self.mountpoint,
            subscriber: AppSubscriber { tx: &self.tx },
        }
        .diff_nodes(nodes, &mut self.vnodes)?;
        Ok(())
    }
}

async fn do_fetch<T>(window: &web::Window, url: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let mut opts = web::RequestInit::new();
    opts.method("GET");
    opts.mode(web::RequestMode::Cors);

    let request = web::Request::new_with_str_and_init(&url, &opts)
        .map_err(|err| runtime_error(&err, "failed to construct Request"))?;
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")
        .map_err(|err| runtime_error(&err, "failed to set Accept header to Request"))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| runtime_error(&err, "failed to fetch request"))?;

    let resp: web::Response = resp_value
        .dyn_into()
        .map_err(|err| runtime_error(&err, "invalid object"))?;

    if !resp.ok() {
        return Err(format!(
            "failed to fetch: {} {}",
            resp.status(),
            resp.status_text()
        ));
    }

    let json = JsFuture::from(
        resp.json()
            .map_err(|err| runtime_error(&err, "before receiving JSON payload"))?,
    )
    .await
    .map_err(|err| runtime_error(&err, "during receiving JSON payload"))?;

    let decoded = json
        .into_serde()
        .map_err(|err| format!("invalid JSON format: {}", err))?;

    Ok(decoded)
}

fn runtime_error(v: &JsValue, msg: &str) -> String {
    if let Some(s) = v.as_string() {
        format!("{}: {}", msg, s)
    } else {
        msg.into()
    }
}

struct AppSubscriber<'a, TMsg: 'static> {
    tx: &'a mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Subscriber for AppSubscriber<'_, TMsg> {
    type Msg = TMsg;
    type Mailbox = AppMailbox<TMsg>;

    #[inline]
    fn mailbox(&self) -> Self::Mailbox {
        AppMailbox {
            tx: self.tx.clone(),
        }
    }
}

struct AppMailbox<TMsg> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg: 'static> Mailbox for AppMailbox<TMsg> {
    type Msg = TMsg;

    fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }
}
