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
    pub fn send(&self, msg: TMsg) {
        let _ = self.tx.unbounded_send(msg);
    }

    pub fn sender<T>(&self, f: impl Fn(T) -> TMsg + 'static) -> impl Fn(T) + 'static
    where
        TMsg: 'static,
    {
        let mailbox = self.clone();
        move |arg| {
            mailbox.send(f(arg));
        }
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
