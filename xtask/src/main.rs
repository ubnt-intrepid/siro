use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
    setup_path()?;

    let examples_dir = project_root().join("examples");

    if let Some(name) = env::args_os().nth(1) {
        let manifest_dir = examples_dir.join(name);
        build(&manifest_dir)?;
        return Ok(());
    }

    let entries = WalkDir::new(&examples_dir)
        .max_depth(2)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_name() == "Cargo.toml" {
                Some(entry)
            } else {
                None
            }
        });

    for entry in entries {
        let manifest_dir = entry.path().parent().unwrap();
        build(&manifest_dir)?;
    }

    Ok(())
}

fn setup_path() -> anyhow::Result<()> {
    if let Some(paths) = env::var_os("PATH") {
        let node_bins = project_root()
            .join("node_modules")
            .join(".bin")
            .canonicalize()?;

        let new_paths = env::join_paths(
            Some(node_bins) //
                .into_iter()
                .chain(env::split_paths(&paths)),
        )?;

        env::set_var("PATH", &new_paths);
    }

    Ok(())
}

fn build(manifest_dir: &Path) -> anyhow::Result<()> {
    let webpack_config = manifest_dir.join("webpack.config.js");
    if webpack_config.is_file() {
        let mut webpack = command("webpack");
        webpack.arg("--config").arg(&webpack_config);

        println!("[xtask] run command: {:?}", webpack);
        let status = webpack.status()?;
        anyhow::ensure!(status.success(), "webpack failed with: {:?}", status);

        return Ok(());
    }

    let mut wasm_pack = command("wasm-pack");
    wasm_pack
        .arg("build")
        .arg("--dev")
        .arg("--no-typescript")
        .args(&["--target", "web"])
        .args(&["--out-name", "index"]);
    wasm_pack.arg("--out-dir").arg(manifest_dir.join("pkg"));
    wasm_pack.arg(&manifest_dir);

    println!("[xtask] run command: {:?}", wasm_pack);
    let status = wasm_pack.status()?;
    anyhow::ensure!(status.success(), "wasm-pack failed with: {:?}", status);

    Ok(())
}

fn command(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    command
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command
}

fn project_root() -> PathBuf {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|| option_env!("CARGO_MANIFEST_DIR").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("./xtask"));
    manifest_dir.parent().unwrap().to_owned()
}
