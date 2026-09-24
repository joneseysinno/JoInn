//! Classify a `.rs` path as facade, capsule root, or leaf.

use std::fs;
use std::path::Path;

/// Kind of file under the module layout rules.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileKind {
    FacadeLib,
    FacadeMain,
    CapsuleRoot,
    Leaf,
}

/// Capsule root requires a sibling folder that contains at least one `.rs` file.
pub fn classify(path: &Path) -> FileKind {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name == "lib.rs" {
        return FileKind::FacadeLib;
    }
    if name == "main.rs" {
        return FileKind::FacadeMain;
    }
    // Capsule root: foo.rs beside foo/ that holds ≥1 `.rs` file.
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(parent) = path.parent() {
            let folder = parent.join(stem);
            if folder.is_dir() {
                let Ok(entries) = fs::read_dir(&folder) else {
                    return FileKind::Leaf;
                };
                let mut names: Vec<_> = entries.flatten().map(|e| e.path()).collect();
                names.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
                if names
                    .iter()
                    .any(|p| p.extension().is_some_and(|e| e == "rs"))
                {
                    return FileKind::CapsuleRoot;
                }
            }
        }
    }
    FileKind::Leaf
}
