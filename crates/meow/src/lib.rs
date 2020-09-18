#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

mod app;
mod mailbox;
mod meow;

pub mod vdom;

pub use crate::{
    app::App, //
    mailbox::Mailbox,
    meow::Meow,
};
