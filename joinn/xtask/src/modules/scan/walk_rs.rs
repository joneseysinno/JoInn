//! Walk a directory tree of `.rs` files in sorted order.

use std::fs;
use std::path::Path;

/// Walk a directory tree of `.rs` files, visiting each directory's entries sorted by name.
pub fn walk_rs(dir: &Path, f: &mut impl FnMut(&Path, &str)) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    let mut paths: Vec<_> = entries
        .map(|e| e.map_err(|err| err.to_string()).map(|ent| ent.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    for path in paths {
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            walk_rs(&path, f)?;
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            f(&path, &text);
        }
    }
    Ok(())
}
