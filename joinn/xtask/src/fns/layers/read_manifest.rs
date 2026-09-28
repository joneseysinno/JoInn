//! Read one `Cargo.toml` textually: its name, its dependency names, its lints.

use std::fs;
use std::path::Path;

use super::Manifest;

/// One `name = …` per line under `[dependencies]`; `workspace = true` under `[lints]`.
pub(crate) fn read_manifest(path: &Path) -> Result<Manifest, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut section = String::new();
    let mut name = None;
    let mut deps = Vec::new();
    let mut lints_workspace = false;
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if t.starts_with('[') {
            section = t.to_owned();
            continue;
        }
        let Some((key, value)) = t.split_once('=') else {
            continue;
        };
        let key = key.trim();
        match section.as_str() {
            "[package]" if key == "name" => {
                name = Some(value.trim().trim_matches('"').to_owned());
            }
            "[dependencies]" => {
                let dep = key.split('.').next().unwrap_or(key).trim();
                deps.push(dep.to_owned());
            }
            "[lints]" if key == "workspace" && value.trim() == "true" => {
                lints_workspace = true;
            }
            _ => {}
        }
    }
    let name = name.ok_or_else(|| format!("{}: no [package] name", path.display()))?;
    Ok(Manifest {
        name,
        deps,
        lints_workspace,
    })
}
