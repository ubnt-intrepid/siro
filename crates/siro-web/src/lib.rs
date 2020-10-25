/*!
A browser runtime for `siro` using `web-sys`.
!*/

#![doc(html_root_url = "https://docs.rs/siro-web/0.1.0")]
#![forbid(unsafe_code, clippy::todo, clippy::unimplemented)]

mod app;
mod render;

pub mod subscription;

pub use crate::app::App;

fn document() -> Option<web::Document> {
    web::window()?.document()
}
