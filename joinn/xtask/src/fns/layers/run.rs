//! Run the layers fixture, then check every workspace member.

use std::fs;

use super::check_layers::check_layers;
use super::members::members;
use super::read_manifest::read_manifest;
use crate::fns::workspace_root::workspace_root;

/// The fixture must be refused before the workspace is judged.
pub(crate) fn layers() -> Result<(), String> {
    let root = workspace_root()?;

    let fixture_dir = root.join("xtask").join("layers_fixtures").join("illegal");
    let mut dirs: Vec<_> = fs::read_dir(&fixture_dir)
        .map_err(|e| format!("{}: {e}", fixture_dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("Cargo.toml").is_file())
        .collect();
    dirs.sort();
    let mut fixture = Vec::new();
    for dir in &dirs {
        fixture.push(read_manifest(&dir.join("Cargo.toml"))?);
    }
    let planted = "layers: joinn-visual -> joinn-gpu";
    if !check_layers(&fixture)
        .iter()
        .any(|h| h.starts_with(planted))
    {
        return Err(
            "layers fixture: joinn-visual -> joinn-gpu was not refused; the check is blind".into(),
        );
    }
    println!("layers fixture: joinn-visual -> joinn-gpu refused (ok)");

    let mut manifests = Vec::new();
    for member in members(&root)? {
        manifests.push(read_manifest(&member.join("Cargo.toml"))?);
    }
    let hits = check_layers(&manifests);
    if !hits.is_empty() {
        return Err(format!(
            "layers: {} hit(s)\n{}",
            hits.len(),
            hits.join("\n")
        ));
    }
    let users = |dep: &str| -> String {
        let mut names: Vec<&str> = manifests
            .iter()
            .filter(|m| m.deps.iter().any(|d| d == dep))
            .map(|m| m.name.as_str())
            .collect();
        names.sort_unstable();
        names.join(", ")
    };
    println!(
        "layers: ok ({} crate(s); wgpu in {}; winit in {})",
        manifests.len(),
        users("wgpu"),
        users("winit")
    );
    Ok(())
}
