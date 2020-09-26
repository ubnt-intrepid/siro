#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

mod app;

pub mod builder;
pub mod html;
pub mod mailbox;
pub mod subscription;
pub mod svg;
pub mod util;
pub mod vdom;

pub use crate::{
    app::{App, Mountpoint}, //
    mailbox::Mailbox,
    vdom::VNode,
};

pub mod prelude {
    pub use crate::builder::Element as _;
    pub use crate::mailbox::Mailbox as _;
}
