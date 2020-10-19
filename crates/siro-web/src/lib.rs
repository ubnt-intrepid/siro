mod app;

pub mod subscription;

pub use crate::app::App;

fn document() -> Option<web::Document> {
    web::window()?.document()
}
