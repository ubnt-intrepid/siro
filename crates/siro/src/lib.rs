pub mod attr;
pub mod children;
pub mod event;
pub mod mailbox;
pub mod node;
pub mod subscription;
pub mod types;

pub mod prelude {
    pub use crate::{
        attr::Attr,
        children::Children,
        event::{Event, EventDecoder},
        mailbox::Mailbox,
        node::{IntoNode, Node},
        subscription::{Subscribe, Subscription},
    };
}
