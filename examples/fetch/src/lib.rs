use siro::html::{
    attr::{placeholder, value},
    button, div,
    event::{on_click, on_input},
    input,
};
use siro::prelude::*;

use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

struct Model {
    repo_slug: String,
    branch: String,
    response: Option<Result<Branch, String>>,
}

// ==== update ====

enum Msg {
    UpdateRepoSlug(String),
    UpdateBranch(String),
    RequestFetch,
    UpdateResponse(Result<Branch, String>),
}

enum Cmd {
    Fetch(String),
    None,
}

fn update(model: &mut Model, msg: Msg) -> Cmd {
    match msg {
        Msg::UpdateRepoSlug(slug) => {
            model.repo_slug = slug;
            Cmd::None
        }

        Msg::UpdateBranch(branch) => {
            model.branch = branch;
            Cmd::None
        }

        Msg::RequestFetch => {
            let repo_slug = model.repo_slug.trim();
            let mut branch = model.branch.trim();
            if branch.is_empty() {
                branch = "master";
            }
            let url = format!(
                "https://api.github.com/repos/{}/branches/{}",
                repo_slug, branch
            );
            Cmd::Fetch(url)
        }

        Msg::UpdateResponse(response) => {
            model.response.replace(response);
            Cmd::None
        }
    }
}

// ==== view ====

fn view(model: &Model) -> impl Nodes<Msg> {
    div(
        (),
        (
            div(
                (),
                (
                    input::text((
                        placeholder("Repository"),
                        value(model.repo_slug.clone()),
                        on_input(Msg::UpdateRepoSlug),
                    )),
                    input::text((
                        placeholder("Branch (default: master)"),
                        value(model.branch.clone()),
                        on_input(Msg::UpdateBranch),
                    )),
                    button(on_click(|| Msg::RequestFetch), "Fetch"),
                ),
            ), //
            div(
                (),
                match model.response {
                    Some(Ok(ref branch)) => format!("branch: {:?}", branch).into(),
                    Some(Err(ref error)) => format!("error: {:?}", error).into(),
                    None => None,
                },
            ),
        ),
    )
}

// ==== runtime ====

#[derive(Debug, Deserialize)]
struct Branch {
    name: String,
    commit: Commit,
}

#[derive(Debug, Deserialize)]
struct Commit {
    sha: String,
    commit: CommitDetails,
}

#[derive(Debug, Deserialize)]
struct CommitDetails {
    author: Signature,
    committer: Signature,
}

#[derive(Debug, Deserialize)]
struct Signature {
    name: String,
    email: String,
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut app = siro_web::App::new()?;
    app.mount("#app")?;

    let mut model = Model {
        repo_slug: "ubnt-intrepid/siro".into(),
        branch: "main".into(),
        response: None,
    };

    app.render(view(&model))?;

    while let Some(msg) = app.next_message().await {
        let cmd = update(&mut model, msg);
        app.render(view(&model))?;

        match cmd {
            Cmd::Fetch(url) => {
                app.start_fetch(url, Msg::UpdateResponse);
            }
            Cmd::None => (),
        }
    }

    Ok(())
}
