mod app;

pub mod attr;
pub mod children;
pub mod event;
pub mod mailbox;
pub mod subscription;
pub mod util;
pub mod vdom;

#[doc(inline)]
pub use crate::{
    app::App, //
    mailbox::Mailbox,
    subscription::Subscription,
    vdom::Node,
};

/// A *prelude* for end users.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        mailbox::{Mailbox, MailboxExt as _},
        subscription::{Subscription, SubscriptionExt as _},
        vdom::{Attr, Children, Node},
    };
}
