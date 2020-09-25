use std::{marker::PhantomData, rc::Rc};

pub trait Mailbox<TMsg> {
    type Sender: Sender<TMsg>;

    fn sender(&self) -> Self::Sender;

    fn map<F>(self, f: F) -> Map<Self, TMsg, F>
    where
        Self: Sized,
    {
        Map {
            mailbox: self,
            f: Rc::new(f),
            _marker: PhantomData,
        }
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
}

pub trait Sender<TMsg>: Clone + 'static {
    fn send_message(&self, msg: TMsg);
}

// ==== Map ====

pub struct Map<M, TMsg, F> {
    mailbox: M,
    f: Rc<F>,
    _marker: PhantomData<TMsg>,
}

impl<M, TMsg, F, UMsg> Mailbox<UMsg> for Map<M, TMsg, F>
where
    M: Mailbox<TMsg>,
    TMsg: 'static,
    F: Fn(UMsg) -> TMsg + 'static,
{
    type Sender = MapSender<M::Sender, TMsg, F>;

    fn sender(&self) -> Self::Sender {
        MapSender {
            sender: self.mailbox.sender(),
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

pub struct MapSender<M, TMsg, F> {
    sender: M,
    f: Rc<F>,
    _marker: PhantomData<TMsg>,
}

impl<M, TMsg, F> Clone for MapSender<M, TMsg, F>
where
    M: Sender<TMsg>,
{
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            f: self.f.clone(),
            _marker: PhantomData,
        }
    }
}

impl<M, TMsg, F, UMsg> Sender<UMsg> for MapSender<M, TMsg, F>
where
    M: Sender<TMsg>,
    TMsg: 'static,
    F: Fn(UMsg) -> TMsg + 'static,
{
    fn send_message(&self, msg: UMsg) {
        self.sender.send_message((self.f)(msg));
    }
}
