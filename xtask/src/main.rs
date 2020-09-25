use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};
use walkdir::WalkDir;

fn main() -> anyhow::Result<()> {
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

    let entries = WalkDir::new(project_root().join("examples"))
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

        let mut wasm_pack = Command::new("wasm-pack");
        wasm_pack
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
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
    }

    Ok(())
}

fn project_root() -> PathBuf {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|| option_env!("CARGO_MANIFEST_DIR").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("./xtask"));
    manifest_dir.parent().unwrap().to_owned()
}
