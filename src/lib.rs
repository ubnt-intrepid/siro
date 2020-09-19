#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

mod app;
mod global;
mod mailbox;

pub mod util;
pub mod vdom;

pub use crate::{
    app::App, //
    global::{global_context, GlobalContext},
    mailbox::{mailbox, Mailbox, Mails},
};
