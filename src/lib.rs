/*!
A library for building client side Web applications.

This crate provides the *representation* of Web applications and virtual DOMs
written by the authors.  The applications are not tied to a particular
environment, such as the browser or the server side, and are separate from the
detail of runtime implementation.
!*/

#![doc(html_root_url = "https://docs.rs/siro/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

pub mod effects;
pub mod html;
pub mod svg;
pub mod vdom;

/// A *prelude* for end users.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::vdom::{Attributes, Nodes};
}
