#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

mod app;
mod callback;
mod global;
mod mailbox;

pub mod builder;
pub mod util;
pub mod vdom;

pub use crate::{
    app::App, //
    callback::Callback,
    mailbox::{mailbox, Mailbox, Mails},
};
