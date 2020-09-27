mod map;

pub use map::Map;

/// Represents mailbox for exchanging the messages within the application.
pub trait Mailbox {
    type Msg: 'static;

    /// The type of `Sender` associated with this mailbox.
    type Sender: Sender<Msg = Self::Msg>;

    /// Create an instance of `Sender` associated with this mailbox.
    fn sender(&self) -> Self::Sender;

    /// Post a message to this mailbox.
    ///
    /// By default, this method is a shortcut to `self.sender().send_message(msg)`.
    fn send_message(&self, msg: Self::Msg) {
        self.sender().send_message(msg);
    }

    /// Create a proxy `Mailbox` to receive other type of messages.
    fn map<F, TMsg>(self, f: F) -> Map<Self, F, TMsg>
    where
        Self: Sized,
        F: Fn(TMsg) -> Self::Msg + Clone + 'static,
        TMsg: 'static,
    {
        Map::new(self, f)
    }
}

impl<T: ?Sized> Mailbox for &T
where
    T: Mailbox,
{
    type Msg = T::Msg;
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: Self::Msg) {
        (**self).send_message(msg);
    }
}

impl<T: ?Sized> Mailbox for Box<T>
where
    T: Mailbox,
{
    type Msg = T::Msg;
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: Self::Msg) {
        (**self).send_message(msg);
    }
}

impl<T: ?Sized> Mailbox for std::rc::Rc<T>
where
    T: Mailbox,
{
    type Msg = T::Msg;
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: Self::Msg) {
        (**self).send_message(msg);
    }
}

impl<T: ?Sized> Mailbox for std::sync::Arc<T>
where
    T: Mailbox,
{
    type Msg = T::Msg;
    type Sender = T::Sender;

    #[inline]
    fn sender(&self) -> Self::Sender {
        (**self).sender()
    }

    #[inline]
    fn send_message(&self, msg: Self::Msg) {
        (**self).send_message(msg);
    }
}

/// Sender for posting messages from callback functions.
pub trait Sender: 'static {
    type Msg: 'static;

    /// Send a message to the mailbox.
    fn send_message(&self, msg: Self::Msg);
}
