use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use siro::prelude::*;
use siro::{
    html::{
        self, attr,
        event::{on_blur, on_click, on_double_click, on_enter, on_input},
    },
    vdom::class,
};
use std::str::FromStr;

// ==== model ====

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Model {
    #[serde(skip)]
    pub input: String,
    pub entries: IndexMap<TodoId, TodoEntry>,
    #[serde(skip)]
    pub visibility: Option<Visibility>,
}

pub type TodoId = ulid::Ulid;

fn todo_edit_input_id(id: TodoId) -> String {
    format!("todo-{}", id)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoEntry {
    pub id: TodoId,
    pub description: String,
    pub completed: bool,
    #[serde(skip)]
    pub editing: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Visibility {
    All,
    Active,
    Completed,
}

impl FromStr for Visibility {
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

pub enum Msg {
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

pub trait Effects {
    type Ok;
    type Error;

    fn save_model(&mut self) -> Result<(), Self::Error>;
    fn focus_element(&mut self, id: &str) -> Result<(), Self::Error>;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub fn update<E>(model: &mut Model, msg: Msg, mut effects: E) -> Result<E::Ok, E::Error>
where
    E: Effects,
{
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
                effects.save_model()?;
            }
        }

        Msg::Check(id, completed) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.completed = completed;
                effects.save_model()?;
            }
        }

        Msg::CheckAll(completed) => {
            for entry in model.entries.values_mut() {
                entry.completed = completed;
            }
            effects.save_model()?;
        }

        Msg::Delete(id) => {
            if let Some(..) = model.entries.remove(&id) {
                effects.save_model()?;
            }
        }

        Msg::DeleteCompleted => {
            model.entries.retain(|_, entry| !entry.completed);
            effects.save_model()?;
        }

        Msg::ChangeVisibility(visibility) => {
            model.visibility.replace(visibility);
        }

        Msg::UpdateEntry(id, description) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.description = description;
                effects.save_model()?;
            }
        }

        Msg::EditingEntry(id, editing) => {
            if let Some(entry) = model.entries.get_mut(&id) {
                entry.editing = editing;
            }

            if editing {
                effects.focus_element(&todo_edit_input_id(id))?;
            }
        }
    }

    effects.end()
}

// ==== view ====

pub fn view(model: &Model) -> impl Nodes<Msg> + '_ {
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
                siro::vdom::iter(
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

fn if_then<T>(pred: bool, f: impl FnOnce() -> T) -> Option<T> {
    if pred {
        Some(f())
    } else {
        None
    }
}
