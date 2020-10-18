mod app;

pub mod mailbox;
pub mod subscription;
pub mod util;

#[doc(inline)]
pub use crate::{
    app::App, //
    mailbox::Mailbox,
    subscription::Subscription,
};

#[doc(no_inline)]
pub use siro_vdom::{self as vdom, attr, children};

/// A *prelude* for end users.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{
        mailbox::{Mailbox, MailboxExt as _},
        subscription::{Subscription, SubscriptionExt as _},
    };

    #[doc(no_inline)]
    pub use siro_vdom::{attr::Attr, children::Children, node::Node};
}
