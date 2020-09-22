use crate::callback::Callback;
use futures::{channel::mpsc, prelude::*};

pub fn mailbox<TMsg>() -> (Mailbox<TMsg>, Mails<TMsg>) {
    let (tx, rx) = mpsc::unbounded();
    (Mailbox { tx }, Mails { rx })
}

pub struct Mailbox<TMsg> {
    tx: mpsc::UnboundedSender<TMsg>,
}

impl<TMsg> Clone for Mailbox<TMsg> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<TMsg> Mailbox<TMsg> {
    pub fn send_message(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }

    pub fn callback<T>(&self, f: impl Fn(T) -> TMsg + 'static) -> Callback<T>
    where
        TMsg: 'static,
    {
        let mailbox = self.clone();
        Callback::from(move |arg| {
            mailbox.send_message(f(arg));
        })
    }
}

pub struct Mails<TMsg> {
    rx: mpsc::UnboundedReceiver<TMsg>,
}

impl<TMsg> Mails<TMsg> {
    pub async fn recv(&mut self) -> Option<TMsg> {
        self.rx.next().await
    }
}
