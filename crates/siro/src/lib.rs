mod app;

pub mod mailbox;
pub mod subscription;
pub mod util;

#[doc(inline)]
pub use crate::{
    app::App, //
    mailbox::Mailbox,
    subscription::Subscribe,
};

#[doc(no_inline)]
pub use {
    siro_html as html, //
    siro_svg as svg,
    siro_vdom::{self as vdom, attr, children},
};

/// A *prelude* for end users.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{mailbox::Mailbox, subscription::Subscribe};

    #[doc(no_inline)]
    pub use siro_vdom::{attr::Attr, children::Children, node::Node};
}
