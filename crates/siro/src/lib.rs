mod app;

pub mod attr;
pub mod children;
pub mod event;
pub mod mailbox;
pub mod subscription;
pub mod util;
pub mod vdom;
pub mod view;

#[doc(inline)]
pub use crate::{
    app::App, //
    children::Children,
    mailbox::Mailbox,
    subscription::Subscription,
    view::View,
};

/// A *prelude* for end users.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        attr::Attr,
        children::Children,
        mailbox::{Mailbox, MailboxExt as _},
        subscription::{Subscription, SubscriptionExt as _},
        view::{View, ViewExt as _},
    };
}
