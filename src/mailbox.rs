mod proxy;

pub use proxy::proxy;

/// Represents mailbox for exchanging the messages within the application.
pub trait Mailbox<TMsg> {
    /// The type of `Sender` associated with this mailbox.
    type Sender: Sender<TMsg>;

    /// Create an instance of `Sender` associated with this mailbox.
    fn sender(&self) -> Self::Sender;

    /// Post a message to this mailbox.
    ///
    /// By default, this method is a shortcut to `self.sender().send_message(msg)`.
    fn send_message(&self, msg: TMsg) {
        self.sender().send_message(msg);
    }
}

impl<T: ?Sized, TMsg> Mailbox<TMsg> for &T
where
    T: Mailbox<TMsg>,
{
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: TMsg) {
        (**self).send_message(msg);
    }
}

impl<T: ?Sized, TMsg> Mailbox<TMsg> for Box<T>
where
    T: Mailbox<TMsg>,
{
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: TMsg) {
        (**self).send_message(msg);
    }
}

impl<T: ?Sized, TMsg> Mailbox<TMsg> for std::rc::Rc<T>
where
    T: Mailbox<TMsg>,
{
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: TMsg) {
        (**self).send_message(msg);
    }
}

impl<T: ?Sized, TMsg> Mailbox<TMsg> for std::sync::Arc<T>
where
    T: Mailbox<TMsg>,
{
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: TMsg) {
        (**self).send_message(msg);
    }
}

/// Sender for posting messages from callback functions.
pub trait Sender<TMsg>: 'static {
    /// Send a message to the mailbox.
    fn send_message(&self, msg: TMsg);
}
