/*!
The representation of subscriptions.
!*/

mod map;

pub use map::Map;

/// Representing a subscription.
pub trait Subscription {
    /// The message type produced by this subscription.
    type Msg: 'static;

    /// The error type from this subscription.
    type Error;

    /// The session type returned from `subscribe`.
    type Subscribe: Subscribe<Error = Self::Error>;

    /// Register this subscription to the specific context.
    fn subscribe<S>(self, subscriber: S) -> Result<Self::Subscribe, Self::Error>
    where
        S: Subscriber<Msg = Self::Msg>;

    /// Map the message type to another one.
    fn map<F, TMsg>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Msg) -> TMsg + Clone + 'static,
        TMsg: 'static,
    {
        Map::new(self, f)
    }
}

/// The session until the end of subscription.
pub trait Subscribe {
    /// The error type returned from `unsubscribe`.
    type Error;

    /// Stop this subscription.
    fn unsubscribe(&mut self) -> Result<(), Self::Error>;
}

/// Representing the subscriber of messages.
pub trait Subscriber {
    /// The message type associated with this context.
    type Msg: 'static;

    /// The type of mailbox returned from `mailbox`.
    type Mailbox: Mailbox<Msg = Self::Msg>;

    /// Create an instance of mailbox.
    fn mailbox(&self) -> Self::Mailbox;
}

impl<S: ?Sized> Subscriber for &S
where
    S: Subscriber,
{
    type Msg = S::Msg;
    type Mailbox = S::Mailbox;

    #[inline]
    fn mailbox(&self) -> Self::Mailbox {
        (*self).mailbox()
    }
}

/// A mailbox for sending messages to the subscriber.
pub trait Mailbox: 'static {
    /// The message type to be sent.
    type Msg: 'static;

    /// Send a message value to the subscriber.
    fn send_message(&self, msg: Self::Msg);
}
