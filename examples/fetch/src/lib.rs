use siro::html::{
    attr::{placeholder, value},
    button, div,
    event::{on_click, on_input},
    input,
};
use siro::prelude::*;

use futures::prelude::*;
use futures::stream::FuturesUnordered;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast as _;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
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
    Fetch { repo_slug: String, branch: String },
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
            let repo_slug = model.repo_slug.trim().to_owned();
            let mut branch = model.branch.trim().to_owned();
            if branch.is_empty() {
                branch = "master".into();
            }
            Cmd::Fetch { repo_slug, branch }
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

    let window = web_sys::window().ok_or("no global Window exists")?;

    let mut app = siro_web::App::mount("#app")?;

    let mut model = Model {
        repo_slug: "ubnt-intrepid/siro".into(),
        branch: "main".into(),
        response: None,
    };

    app.render(view(&model))?;

    let mut cmd_tasks = FuturesUnordered::new();
    loop {
        futures::select! {
            msg = app.select_next_some() => {
                let cmd = update(&mut model, msg);
                app.render(view(&model))?;

                match cmd {
                    Cmd::Fetch { repo_slug, branch } => {
                        let window = window.clone();
                        cmd_tasks.push(async move {
                            let response = do_fetch(&window, &repo_slug, &branch).await;
                            Msg::UpdateResponse(response)
                        });
                    }
                    Cmd::None => (),
                }
            },

            msg = cmd_tasks.select_next_some() => {
                app.send_message(msg);
            }

            complete => break,
        }
    }

    Ok(())
}

async fn do_fetch(
    window: &web_sys::Window,
    repo_slug: &str,
    branch: &str,
) -> Result<Branch, String> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!(
        "https://api.github.com/repos/{}/branches/{}",
        repo_slug, branch
    );

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|err| runtime_error(&err, "failed to construct Request"))?;
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")
        .map_err(|err| runtime_error(&err, "failed to set Accept header to Request"))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|err| runtime_error(&err, "failed to fetch request"))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|err| runtime_error(&err, "invalid object"))?;

    if !resp.ok() {
        return Err(format!(
            "failed to fetch: {} {}",
            resp.status(),
            resp.status_text()
        ));
    }

    let json = JsFuture::from(
        resp.json()
            .map_err(|err| runtime_error(&err, "before receiving JSON payload"))?,
    )
    .await
    .map_err(|err| runtime_error(&err, "during receiving JSON payload"))?;

    let branch_info: Branch = json
        .into_serde()
        .map_err(|err| format!("invalid JSON format: {}", err))?;

    Ok(branch_info)
}

fn runtime_error(v: &JsValue, msg: &str) -> String {
    if let Some(s) = v.as_string() {
        format!("{}: {}", msg, s)
    } else {
        msg.into()
    }
}
