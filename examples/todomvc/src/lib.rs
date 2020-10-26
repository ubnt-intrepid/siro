use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use siro::attr::class;
use siro::prelude::*;
use siro_html::{
    self as html, attr,
    event::{on_blur, on_click, on_double_click, on_enter, on_input},
};
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

enum Command {
    SaveModel,
    FocusTodo(TodoId),
    Nop,
}

fn update(model: &mut Model, msg: Msg) -> Command {
    match msg {
        Msg::UpdateField(input) => {
            model.input = input;
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
                return Command::SaveModel;
            }
        }

        Msg::Check(id, completed) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.completed = completed;
                return Command::SaveModel;
            }
        }

        Msg::CheckAll(completed) => {
            for entry in model.entries.values_mut() {
                entry.completed = completed;
            }
            return Command::SaveModel;
        }

        Msg::Delete(id) => {
            if let Some(..) = model.entries.remove(&id) {
                return Command::SaveModel;
            }
        }

        Msg::DeleteCompleted => {
            model.entries.retain(|_, entry| !entry.completed);
            return Command::SaveModel;
        }

        Msg::ChangeVisibility(visibility) => {
            model.visibility.replace(visibility);
        }

        Msg::UpdateEntry(id, description) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.description = description;
                return Command::SaveModel;
            }
        }

        Msg::EditingEntry(id, editing) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.editing = editing;
            }

            if editing {
                return Command::FocusTodo(id);
            }
        }
    }

    Command::Nop
}

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> + '_ {
    html::div(
        class("todomvc-wrapper"),
        (
            html::section(
                class("todoapp"),
                (
                    view_header(model),
                    view_main(model),
                    if_then(!model.entries.is_empty(), || view_controls(model)),
                ),
            ),
            view_info_footer(),
        ),
    )
}

fn view_header(model: &Model) -> impl Nodes<Msg> {
    html::header(
        class("header"),
        (
            html::h1((), "todos"),
            html::input(
                (
                    class("new-todo"),
                    attr::placeholder("What needs to be done?"),
                    attr::autofocus(true),
                    attr::name("new_todo"),
                    attr::value(model.input.clone()),
                    on_input(Msg::UpdateField),
                    on_enter(|| Msg::Add),
                ),
                (),
            ),
        ),
    )
}

fn view_main(model: &Model) -> impl Nodes<Msg> + '_ {
    let all_completed = model.entries.values().all(|entry| entry.completed);

    html::section(
        class("main"),
        (
            html::input::checkbox((
                class("toggle-all"),
                attr::id("toggle-all"),
                attr::checked(all_completed),
                on_click(move || Msg::CheckAll(!all_completed)),
            )),
            html::label(attr::label_for("toggle-all"), "Mark all as complete"),
            html::ul(
                class("todo-list"),
                siro::node::iter(
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

fn view_entry(entry: &TodoEntry) -> impl Nodes<Msg> {
    let TodoEntry {
        id,
        completed,
        editing,
        ref description,
        ..
    } = *entry;

    html::li(
        (
            if_then(completed, || class("completed")),
            if_then(editing, || class("editing")),
        ),
        (
            html::div(
                class("view"),
                (
                    html::input::checkbox((
                        class("toggle"),
                        attr::checked(completed),
                        on_click(move || Msg::Check(id, !completed)),
                    )),
                    html::label(
                        on_double_click(move || Msg::EditingEntry(id, true)),
                        description.clone(),
                    ),
                    html::button((class("destroy"), on_click(move || Msg::Delete(id))), ()),
                ),
            ),
            if_then(editing, || {
                html::input::text((
                    class("edit"),
                    attr::name("title"),
                    attr::id(todo_edit_input_id(id)),
                    attr::value(description.clone()),
                    on_input(move |input| Msg::UpdateEntry(id, input)),
                    on_blur(move || Msg::EditingEntry(id, false)),
                    on_enter(move || Msg::EditingEntry(id, false)),
                ))
            }),
        ),
    )
}

fn view_controls(model: &Model) -> impl Nodes<Msg> {
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
        class("footer"),
        (
            html::span(
                class("todo-count"),
                (
                    html::strong((), entries_left.to_string()),
                    format!(" item{} left", plural_prefix(entries_left)),
                ),
            ),
            html::ul(
                class("filters"),
                (
                    view_visibility_swap(model, Visibility::All, "All", "#/"),
                    view_visibility_swap(model, Visibility::Active, "Active", "#/active"),
                    view_visibility_swap(model, Visibility::Completed, "Completed", "#/completed"),
                ),
            ),
            if_then(has_completed, || {
                html::button(
                    (class("clear-completed"), on_click(|| Msg::DeleteCompleted)),
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
) -> impl Nodes<Msg> {
    let selected = model.visibility.map_or(false, |vis| vis == v);
    html::li(
        html::event::on_click(move || Msg::ChangeVisibility(v)),
        html::a(
            (attr::href(url), if_then(selected, || class("selected"))),
            text,
        ),
    )
}

fn view_info_footer() -> impl Nodes<Msg> {
    html::footer(
        class("info"),
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

fn todo_edit_input_id(id: TodoId) -> String {
    format!("todo-{}", id)
}

fn focus_todo(id: TodoId) -> Option<()> {
    web_sys::window()?
        .document()?
        .get_element_by_id(&todo_edit_input_id(id))?
        .dyn_into::<web_sys::HtmlElement>()
        .ok()?
        .focus()
        .ok()
}

fn if_then<T>(pred: bool, f: impl FnOnce() -> T) -> Option<T> {
    if pred {
        Some(f())
    } else {
        None
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

    let mut app = siro_web::App::mount("#app")?;
    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        let cmd = update(&mut model, msg);
        app.render(view(&model))?;

        match cmd {
            Command::FocusTodo(id) => {
                let _ = focus_todo(id);
            }
            Command::SaveModel => {
                let _ = save_model(&model, &storage);
            }
            Command::Nop => (),
        }
    }

    Ok(())
}
