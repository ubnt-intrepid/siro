#![feature(move_ref_pattern)] // Delete after https://github.com/rust-lang/rust/pull/76119 is merged

pub mod app;
pub mod attr;
pub mod event;
pub mod html;
pub mod mailbox;
pub mod subscription;
pub mod svg;
pub mod util;
pub mod vdom;
pub mod view;

#[doc(inline)]
pub use crate::{
    app::App, //
    mailbox::Mailbox,
    subscription::Subscription,
    view::{Children, View},
};

/// A *prelude* for end users.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        attr::Attr,
        mailbox::{Mailbox, MailboxExt as _},
        subscription::{Subscription, SubscriptionExt as _},
        view::{Children, View, ViewExt as _},
    };
}
