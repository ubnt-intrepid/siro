use futures::prelude::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use siro::prelude::*;
use siro::{attr, event, html, util::if_, App};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use web_sys::Storage;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

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

// ==== update ====

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

// ==== view ====

fn view(model: &Model) -> impl View<Msg = Msg> + '_ {
    html::div(
        attr::class("todomvc-wrapper"),
        (
            html::section(
                attr::class("todoapp"),
                (
                    view_header(model),
                    view_main(model),
                    if_(!model.entries.is_empty(), || view_controls(model)),
                ),
            ),
            view_info_footer(),
        ),
    )
}

fn view_header(model: &Model) -> impl View<Msg = Msg> {
    html::header(
        attr::class("header"),
        (
            html::h1((), "todos"),
            html::input(
                (
                    attr::class("new-todo"),
                    html::attr::placeholder("What needs to be done?"),
                    html::attr::autofocus(true),
                    html::attr::name("new_todo"),
                    html::attr::value(model.input.clone()),
                    event::on_input(Msg::UpdateField),
                    event::on_enter(|| Msg::Add),
                ),
                (),
            ),
        ),
    )
}

fn view_main(model: &Model) -> impl View<Msg = Msg> + '_ {
    let all_completed = model.entries.values().all(|entry| entry.completed);

    html::section(
        attr::class("main"),
        (
            html::input::checkbox(
                (
                    attr::class("toggle-all"),
                    html::attr::id("toggle-all"),
                    html::attr::checked(all_completed),
                    event::on("click", move |_| Msg::CheckAll(!all_completed)),
                ),
                (),
            ),
            html::label(html::attr::label_for("toggle-all"), "Mark all as complete"),
            html::ul(
                attr::class("todo-list"),
                siro::view::iter(
                    model
                        .entries
                        .values()
                        .filter(move |entry| match model.visibility {
                            Some(Visibility::Active) => !entry.completed,
                            Some(Visibility::Completed) => entry.completed,
                            _ => true,
                        })
                        .map(|entry| view_entry(entry)),
                ),
            ),
        ),
    )
}

fn view_entry(entry: &TodoEntry) -> impl View<Msg = Msg> {
    let TodoEntry {
        id: entry_id,
        completed,
        editing,
        ref description,
        ..
    } = *entry;

    let input_id = input_id(entry_id);

    html::li(
        (
            if_(completed, || attr::class("completed")),
            if_(editing, || attr::class("editing")),
        ),
        (
            html::div(
                attr::class("view"),
                (
                    html::input::checkbox(
                        (
                            attr::class("toggle"),
                            html::attr::checked(completed),
                            event::on("click", move |_| Msg::Check(entry_id, !completed)),
                        ),
                        (),
                    ),
                    html::label(
                        event::on("dblclick", move |_| Msg::EditingEntry(entry_id, true)),
                        description.clone(),
                    ),
                    html::button(
                        (
                            attr::class("destroy"),
                            event::on("click", move |_| Msg::Delete(entry_id)),
                        ),
                        (),
                    ),
                ),
            ),
            if_(editing, || {
                html::input::text(
                    (
                        attr::class("edit"),
                        html::attr::name("title"),
                        html::attr::id(input_id.clone()),
                        html::attr::value(description.clone()),
                        event::on_input(move |input| Msg::UpdateEntry(entry_id, input)),
                        event::on("blur", move |_| Msg::EditingEntry(entry_id, false)),
                        event::on_enter(move || Msg::EditingEntry(entry_id, false)),
                    ),
                    (),
                )
            }),
        ),
    )
}

fn view_controls(model: &Model) -> impl View<Msg = Msg> {
    let has_completed = model.entries.values().any(|entry| entry.completed);
    let entries_left = model.entries.values().filter(|e| !e.completed).count();

    fn plural_prefix(n: usize) -> &'static str {
        if n == 1 {
            ""
        } else {
            "s"
        }
    }

    html::footer(
        attr::class("footer"),
        (
            html::span(
                attr::class("todo-count"),
                (
                    html::strong((), entries_left.to_string()),
                    format!(" item{} left", plural_prefix(entries_left)),
                ),
            ),
            html::ul(
                attr::class("filters"),
                (
                    view_visibility_swap(model, Visibility::All, "All", "#/"),
                    view_visibility_swap(model, Visibility::Active, "Active", "#/active"),
                    view_visibility_swap(model, Visibility::Completed, "Completed", "#/completed"),
                ),
            ),
            if_(has_completed, || {
                html::button(
                    (
                        attr::class("clear-completed"),
                        event::on("click", |_| Msg::DeleteCompleted),
                    ),
                    "Clear completed",
                )
            }),
        ),
    )
}

fn view_visibility_swap(
    model: &Model,
    v: Visibility,
    text: &'static str,
    url: &'static str,
) -> impl View<Msg = Msg> {
    let selected = model.visibility.map_or(false, |vis| vis == v);
    html::li(
        event::on("click", move |_| Msg::ChangeVisibility(v)),
        html::a(
            (
                html::attr::href(url),
                if_(selected, || attr::class("selected")),
            ),
            text,
        ),
    )
}

fn view_info_footer() -> impl View<Msg = Msg> {
    html::footer(
        attr::class("info"),
        (
            html::p((), "Double-click to edit a todo"),
            html::p(
                (),
                (
                    "Written by ",
                    html::a(
                        html::attr::href("https://github.com/ubnt-intrepid"),
                        "@ubnt-intrepid",
                    ),
                    html::p(
                        (),
                        (
                            "Part of ",
                            html::a(html::attr::href("http://todomvc.com"), "TodoMVC"),
                        ),
                    ),
                ),
            ),
        ),
    )
}

// ==== misc ====

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

// ==== runtime ====

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
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        let cmd = update(&mut model, msg, &storage);
        app.render(view(&model))?;
        if let Some(cmd) = cmd {
            wasm_bindgen_futures::spawn_local(cmd);
        }
    }

    Ok(())
}
