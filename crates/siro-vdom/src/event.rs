//! Representation of DOM events.

use serde::de::{Deserializer, Error};

/// An abstraction of DOM events.
pub trait Event<'e> {
    /// The type of deserializer returned from `into_deserializer`.
    type Deserializer: Deserializer<'e, Error = Self::Error>;
    /// The error type of deserializer.
    type Error: Error;

    /// Convert itself into a `Deserializer`.
    fn into_deserializer(self) -> Self::Deserializer;
}

/// Decoder of DOM events.
pub trait EventDecoder {
    /// The message type decoded from events.
    type Msg: 'static;

    /// Decode an `Event` to specific message type.
    fn decode_event<'e, E>(&self, event: E) -> Result<Option<Self::Msg>, E::Error>
    where
        E: Event<'e>;
}
