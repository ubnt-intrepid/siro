/*!
A library for building client side Web applications.

This crate provides the *representation* of Web applications and virtual DOMs
written by the authors.  The applications are not tied to a particular
environment, such as the browser or the server side, and are separate from the
detail of runtime implementation.
!*/

#![doc(html_root_url = "https://docs.rs/siro/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

pub mod attr;
pub mod children;
pub mod event;
pub mod mailbox;
pub mod node;
pub mod subscription;
pub mod types;

pub mod prelude {
    pub use crate::{
        attr::Attr,
        children::Children,
        event::{Event, EventDecoder},
        mailbox::Mailbox,
        node::{IntoNode, Node},
        subscription::{Subscribe, Subscription},
    };
}
