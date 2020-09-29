#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

pub mod app;
pub mod event;
pub mod mailbox;
pub mod subscription;
pub mod util;
pub mod vdom;

#[doc(no_inline)]
pub use crate::{
    app::App, //
    event::EventHandler,
    mailbox::Mailbox,
    subscription::Subscription,
    vdom::{Element, VNode},
};

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        event::{ElementEventExt as _, EventHandler as _},
        mailbox::Mailbox as _,
        subscription::Subscription as _,
        vdom::Element as _,
    };
}
