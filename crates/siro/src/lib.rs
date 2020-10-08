pub mod app;
pub mod attr;
pub mod event;
pub mod mailbox;
pub mod subscription;
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
