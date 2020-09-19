use wasm_bindgen::prelude::*;

thread_local!(
    static GLOBAL: Global = init_global().expect_throw("cannot initialize global context")
);

fn init_global() -> Option<Global> {
    let window = web::window()?;
    let document = window.document()?;
    Some(Global { window, document })
}

pub struct Global {
    #[allow(dead_code)]
    pub(crate) window: web::Window,
    pub(crate) document: web::Document,
}

impl Global {
    pub fn with<R>(f: impl FnOnce(&Global) -> R) -> R {
        GLOBAL.with(|g| f(g))
    }
}
