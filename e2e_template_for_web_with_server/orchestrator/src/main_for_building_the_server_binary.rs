//! Compiles the `server` crate so `target/<profile>/server` exists before
//! the orchestrator tries to spawn it. Cargo run only builds this crate's
//! own deps, so `server` has to be asked for explicitly.

use std::path::PathBuf;

pub fn main_for_building_the_server_binary() -> Result<(), String> {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("orchestrator crate has no parent folder")?
        .to_path_buf();
    let status = std::process::Command::new("cargo")
        .args(["build", "-p", "server"])
        .current_dir(&workspace)
        .status()
        .map_err(|e| format!("could not run cargo: {e}"))?;
    if !status.success() {
        return Err(format!("cargo build -p server failed: {status}"));
    }
    Ok(())
}
