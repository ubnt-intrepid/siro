use futures::future::Future;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use siro::{
    event::{on, on_},
    prelude::*,
    App, Mailbox, VNode,
};
use siro_html::{self as html, input::on_input, prelude::*};
use wasm_bindgen::{prelude::*, JsCast as _};
use web_sys::Storage;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

const STORAGE_KEY: &str = "siro-todomvc-save";

fn save_model(model: &Model, storage: &Storage) {
    // ignore errors
    if let Ok(encoded) = serde_json::to_string(model) {
        let _ = storage.set_item(STORAGE_KEY, &encoded);
    }
}

fn restore_model(storage: &Storage) -> Option<Model> {
    let model_raw = storage.get_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&model_raw).ok()
}

fn current_visibility(window: &web_sys::Window) -> Option<Visibility> {
    let hash = window.location().hash().ok()?;
    hash.trim_start_matches("#/").parse().ok()
}

fn input_id(id: TodoId) -> String {
    format!("todo-{}", id)
}

fn focus_input(id: TodoId) {
    let input_id = input_id(id);
    if let Some(document) = siro::util::document() {
        if let Some(element) = document.get_element_by_id(&input_id) {
            let element: web_sys::HtmlElement = element.unchecked_into();
            let _ = element.focus();
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Model {
    #[serde(skip)]
    input: String,
    entries: IndexMap<TodoId, TodoEntry>,
    #[serde(skip)]
    visibility: Option<Visibility>,
}

type TodoId = ulid::Ulid;

#[derive(Debug, Deserialize, Serialize)]
struct TodoEntry {
    id: TodoId,
    description: String,
    completed: bool,
    #[serde(skip)]
    editing: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
enum Visibility {
    All,
    Active,
    Completed,
}

impl std::str::FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.trim().to_lowercase() {
            "all" => Ok(Visibility::All),
            "active" => Ok(Visibility::Active),
            "completed" => Ok(Visibility::Completed),
            _ => Err(()),
        }
    }
}

enum Msg {
    UpdateField(String),
    Add,
    Check(TodoId, bool),
    CheckAll(bool),
    Delete(TodoId),
    DeleteCompleted,
    ChangeVisibility(Visibility),
    UpdateEntry(TodoId, String),
    EditingEntry(TodoId, bool),
}

fn update(
    model: &mut Model,
    msg: Msg,
    storage: &Storage,
) -> Option<impl Future<Output = ()> + 'static> {
    match msg {
        Msg::UpdateField(input) => {
            model.input = input;
            None
        }

        Msg::Add => {
            let description = std::mem::take(&mut model.input).trim().to_owned();
            if !description.is_empty() {
                let id = TodoId::new();
                model.entries.insert(
                    id,
                    TodoEntry {
                        id,
                        description,
                        completed: false,
                        editing: false,
                    },
                );
                save_model(model, storage);
            }
            None
        }

        Msg::Check(id, completed) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.completed = completed;
                save_model(model, storage);
            }
            None
        }

        Msg::CheckAll(completed) => {
            for entry in model.entries.values_mut() {
                entry.completed = completed;
            }
            save_model(model, storage);
            None
        }

        Msg::Delete(id) => {
            if let Some(..) = model.entries.remove(&id) {
                save_model(model, storage);
            }
            None
        }

        Msg::DeleteCompleted => {
            model.entries.retain(|_, entry| !entry.completed);
            save_model(model, storage);
            None
        }

        Msg::ChangeVisibility(visibility) => {
            model.visibility.replace(visibility);
            None
        }

        Msg::UpdateEntry(id, description) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.description = description;
                save_model(model, storage);
            }
            None
        }

        Msg::EditingEntry(id, editing) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.editing = editing;
            }

            if editing {
                Some(async move {
                    focus_input(id);
                })
            } else {
                None
            }
        }
    }
}

fn view(model: &Model, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
    html::div()
        .class("todomvc-wrapper")
        .child(
            html::section()
                .class("todoapp")
                .child(view_header(model, mailbox))
                .child(view_main(model, mailbox))
                .if_(!model.entries.is_empty(), |view| {
                    view.child(view_controls(model, mailbox))
                }),
        )
        .child(view_info_footer())
}

fn view_header(model: &Model, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
    html::header()
        .class("header")
        .child(html::h1().child("todos"))
        .child(
            html::input::text()
                .class("new-todo")
                .placeholder("What needs to be done?")
                .attribute("autofocus", true)
                .name("new_todo")
                .value(model.input.clone())
                .event(mailbox, on_input(Msg::UpdateField))
                .event(mailbox, on_enter(|| Msg::Add)),
        )
}

fn view_main(model: &Model, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
    let all_completed = model.entries.values().all(|entry| entry.completed);

    html::section()
        .class("main")
        .child(
            html::input::checkbox()
                .class("toggle-all")
                .id("toggle-all")
                .property("checked", all_completed)
                .event(mailbox, on("click", move |_| Msg::CheckAll(!all_completed))),
        )
        .child(
            html::label()
                .attribute("for", "toggle-all")
                .child("Mark all as complete"),
        )
        .child(
            html::ul().class("todo-list").append(
                model
                    .entries
                    .values()
                    .filter(|entry| match model.visibility {
                        Some(Visibility::Active) => !entry.completed,
                        Some(Visibility::Completed) => entry.completed,
                        _ => true,
                    })
                    .map(|entry| view_entry(entry, mailbox)),
            ),
        )
}

fn view_entry(entry: &TodoEntry, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
    let TodoEntry {
        id,
        completed,
        editing,
        ref description,
        ..
    } = *entry;

    let input_id = input_id(id);

    html::li()
        .if_(completed, |view| view.class("completed"))
        .child(
            html::div()
                .class("view")
                .child(
                    html::input::checkbox()
                        .class("toggle")
                        .property("checked", completed)
                        .event(mailbox, on_check(move |checked| Msg::Check(id, checked))),
                )
                .child(
                    html::label()
                        .event(
                            mailbox,
                            on("dblclick", move |_| Msg::EditingEntry(id, true)),
                        )
                        .child(description.clone()),
                )
                .child(
                    html::button()
                        .class("destroy")
                        .event(mailbox, on("click", move |_| Msg::Delete(id))),
                ),
        )
        .if_(editing, |view| {
            view.class("editing") //
                .child(
                    html::input::text()
                        .class("edit")
                        .name("title")
                        .id(input_id.clone())
                        .value(description.clone())
                        .event(mailbox, on_input(move |input| Msg::UpdateEntry(id, input)))
                        .event(mailbox, on("blur", move |_| Msg::EditingEntry(id, false)))
                        .event(mailbox, on_enter(move || Msg::EditingEntry(id, false))),
                )
        })
}

fn view_controls(model: &Model, mailbox: &impl Mailbox<Msg = Msg>) -> impl Into<VNode> {
    let has_completed = model.entries.values().any(|entry| entry.completed);
    let entries_left = model.entries.values().filter(|e| !e.completed).count();

    fn plural_prefix(n: usize) -> &'static str {
        if n == 1 {
            ""
        } else {
            "s"
        }
    }

    html::footer()
        .class("footer")
        .child(html::span().class("todo-count").children((
            html::strong().child(entries_left.to_string()),
            format!(" item{} left", plural_prefix(entries_left)),
        )))
        .child(html::ul().class("filters").children((
            view_visibility_swap(model, mailbox, Visibility::All, "All", "#/"),
            view_visibility_swap(model, mailbox, Visibility::Active, "Active", "#/active"),
            view_visibility_swap(
                model,
                mailbox,
                Visibility::Completed,
                "Completed",
                "#/completed",
            ),
        )))
        .if_(has_completed, |view| {
            view.child(
                html::button()
                    .class("clear-completed")
                    .event(mailbox, on("click", |_| Msg::DeleteCompleted))
                    .child("Clear completed"),
            )
        })
}

fn view_visibility_swap(
    model: &Model,
    mailbox: &impl Mailbox<Msg = Msg>,
    v: Visibility,
    text: &'static str,
    url: &'static str,
) -> impl Into<VNode> {
    html::li()
        .event(mailbox, on("click", move |_| Msg::ChangeVisibility(v)))
        .child(
            html::a()
                .attribute("href", url)
                .if_(model.visibility.map_or(false, |vis| vis == v), |view| {
                    view.class("selected")
                })
                .child(text),
        )
}

fn view_info_footer() -> impl Into<VNode> {
    html::footer()
        .class("info")
        .child(
            html::p() //
                .child("Double-click to edit a todo"),
        )
        .child(
            html::p()
                .children((
                    "Written by ",
                    html::a()
                        .attribute("href", "https://github.com/ubnt-intrepid")
                        .child("@ubnt-intrepid"),
                ))
                .child(
                    html::p().children((
                        "Part of ",
                        html::a()
                            .attribute("href", "http://todomvc.com")
                            .child("TodoMVC"),
                    )),
                ),
        )
}

fn on_check<T, TMsg>(f: impl Fn(bool) -> TMsg + 'static) -> impl siro::Event<T, Msg = TMsg>
where
    T: siro::Element,
    TMsg: 'static,
{
    on_("click", move |e: &web_sys::Event| {
        let checked = js_sys::Reflect::get(&&e.target()?, &JsValue::from_str("checked"))
            .ok()?
            .as_bool()?;
        Some(f(checked))
    })
}

fn on_enter<T, TMsg>(f: impl Fn() -> TMsg + 'static) -> impl siro::Event<T, Msg = TMsg>
where
    T: siro::Element,
    TMsg: 'static,
{
    on_("keydown", move |e: &web_sys::Event| {
        let key = js_sys::Reflect::get(e.as_ref(), &JsValue::from_str("key"))
            .ok()?
            .as_string()?;
        match &*key {
            "Enter" => Some(f()),
            _ => None,
        }
    })
}

trait BuilderExt {
    fn if_(self, pred: bool, f: impl FnOnce(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        if pred {
            f(self)
        } else {
            self
        }
    }

    fn if_some<T>(self, v: Option<T>, f: impl FnOnce(Self, T) -> Self) -> Self
    where
        Self: Sized,
    {
        if let Some(v) = v {
            f(self, v)
        } else {
            self
        }
    }
}

impl<T> BuilderExt for T {}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().ok_or("no global `Window` exists")?;
    let storage = window
        .local_storage()?
        .ok_or("cannot access localStorage")?;

    let mut model = restore_model(&storage).unwrap_or_default();
    model.visibility = current_visibility(&window);

    let mut app = App::mount("#app")?;
    app.render(view(&model, &app))?;

    while let Some(msg) = app.next_message().await {
        let cmd = update(&mut model, msg, &storage);
        app.render(view(&model, &app))?;
        if let Some(cmd) = cmd {
            wasm_bindgen_futures::spawn_local(cmd);
        }
    }

    Ok(())
}
