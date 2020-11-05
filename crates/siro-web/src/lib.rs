/*!
A browser runtime for `siro` using `web-sys`.
!*/

#![doc(html_root_url = "https://docs.rs/siro-web/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

mod app;
mod env;
mod error;
mod render;

pub mod subscription;

pub use crate::{
    app::App,
    env::Env,
    error::{Error, Result},
};
