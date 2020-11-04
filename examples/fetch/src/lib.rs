use siro::html::{
    attr::{placeholder, value},
    button, div,
    event::{on_click, on_input},
    input,
};
use siro::prelude::*;

use futures::prelude::*;
use futures::{select, stream::FuturesUnordered};
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// ==== model ====

struct Model {
    repo_slug: String,
    branch: String,
    response: Option<Branch>,
}

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

// ==== update ====

enum Msg {
    UpdateRepoSlug(String),
    UpdateBranch(String),
    RequestFetch,
    UpdateResponse(Branch),
}

// Cmd is a user-defined type that indicates the operations performed
// by the runtime.
//
// Unlike Elm, this type is defined on the application side that is not
// tied to the concrete logic.
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
                    Some(ref branch) => format!("branch: {:?}", branch).into(),
                    None => None,
                },
            ),
        ),
    )
}

// ==== runtime ====

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let env = siro_web::Env::new()?;

    // Start the Web application and mount on the specified DOM node.
    let mut app = env.mount("#app")?;

    // Miscellaneous values used in event loop.
    let client = reqwest::Client::new();
    let mut cmd_tasks = FuturesUnordered::new();

    // Initialize the model and render as DOM tree.
    let mut model = Model {
        repo_slug: "ubnt-intrepid/siro".into(),
        branch: "main".into(),
        response: None,
    };
    app.render(view(&model))?;

    // Run the event loop.
    loop {
        // Receive a message from the environment.
        let msg = select! {
            msg = app.select_next_some() => msg,
            msg = cmd_tasks.select_next_some() => msg,
            complete => break,
        };

        // Apply the message to model and update the view.
        let cmd = update(&mut model, msg);
        app.render(view(&model))?;

        // Dispatch the command returned from application.
        //
        // Here, `cmd` is just a value that is not abstracted like `Cmd<Msg>`,
        // and it is manually dispatched inside the event loop.
        //
        // The dispatch result will be returned to the application side
        // as the value of `Msg`.
        match cmd {
            Cmd::Fetch(url) => {
                let send = client
                    .get(&url)
                    .header("Accept", "application/vnd.github.v3+json")
                    .send();
                cmd_tasks.push(async move {
                    let resp = send.await.unwrap();
                    let text = resp.text().await.unwrap();
                    let branch_info: Branch = serde_json::from_str(&text).unwrap();
                    Msg::UpdateResponse(branch_info)
                });
            }

            Cmd::None => (),
        }
    }

    Ok(())
}
