#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

pub mod app;
pub mod event;
pub mod html;
pub mod mailbox;
pub mod subscription;
pub mod svg;
pub mod util;
pub mod vdom;
pub mod view;

#[doc(no_inline)]
pub use crate::{
    app::App, //
    mailbox::Mailbox,
    subscription::Subscription,
    view::View,
};

pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        mailbox::Mailbox as _,
        subscription::Subscription as _,
        view::{View as _, ViewExt as _},
    };
}
