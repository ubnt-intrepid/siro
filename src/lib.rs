use gloo_events::EventListener;
use meow::{
    vdom::{self, Listener},
    Meow,
};
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast as _};
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let meow = Meow::init()?;

    let node = meow
        .select("#app")
        .ok_or("cannot find `#app` in document")?;

    {
        let node = node.dyn_ref::<web_sys::Element>().unwrap_throw();
        while let Some(child) = node.first_element_child() {
            node.remove_child(&*child)?;
        }
    }

    let mut app = meow.mount(&node)?;

    app.draw(&meow, {
        vdom::element("button") //
            .listener(on_click(|_| {
                #[allow(unused_unsafe)]
                unsafe {
                    web_sys::console::log_1(&"foo".into());
                }
            }))
            .child("foo")
    })?;

    app.draw(&meow, {
        vdom::element("button") //
            .listener(on_click(|_| {
                #[allow(unused_unsafe)]
                unsafe {
                    web_sys::console::log_1(&"bar".into());
                }
            }))
            .child("bar")
    })?;

    std::mem::forget(app);

    Ok(())
}

fn on_click(f: impl Fn(&web_sys::Event) + 'static) -> Rc<dyn Listener> {
    struct OnClick<F>(F);

    impl<F> Listener for OnClick<F>
    where
        F: Fn(&web_sys::Event) + 'static,
    {
        fn event_type(&self) -> &str {
            "click"
        }

        fn attach(self: Rc<Self>, target: &web_sys::EventTarget) -> EventListener {
            EventListener::new(target, "click", move |e| {
                (self.0)(e);
            })
        }
    }

    Rc::new(OnClick(f))
}
