use futures::channel::mpsc;

pub trait Mailbox<TMsg>: Clone {
    fn send_message(&self, msg: TMsg);
}

impl<TMsg> Mailbox<TMsg> for mpsc::UnboundedSender<TMsg> {
    fn send_message(&self, msg: TMsg) {
        let _ = self.unbounded_send(msg);
    }
}
