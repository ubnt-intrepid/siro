//! Representation of DOM events.

use serde::de::{Deserialize, Deserializer, Error};

/// An abstraction of DOM events.
pub trait Event {
    /// The type of deserializer returned from `into_deserializer`.
    type Deserializer: for<'de> Deserializer<'de, Error = Self::Error>;
    /// The error type of deserializer.
    type Error: Error;

    /// Convert itself into a `Deserializer`.
    fn into_deserializer(self) -> Self::Deserializer;

    /// Deserialize the event value to specified type.
    fn decode<T>(self) -> Result<T, Self::Error>
    where
        Self: Sized,
        T: for<'de> Deserialize<'de>,
    {
        T::deserialize(self.into_deserializer())
    }
}

/// Decoder of DOM events.
pub trait EventDecoder {
    /// The message type decoded from events.
    type Msg: 'static;

    /// Decode an `Event` to specific message type.
    fn decode_event<E>(&self, event: E) -> Result<Option<Self::Msg>, E::Error>
    where
        E: Event;
}
