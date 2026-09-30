//! Every corpus file with one extension, in path order.

use std::fs;
use std::path::{Path, PathBuf};

/// The counterfeit corpus is refused on purpose and is left out.
pub(crate) fn corpus_files(corpus: &Path, ext: &str) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    let mut dirs = vec![corpus.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for ent in fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let path = ent.map_err(|e| e.to_string())?.path();
            if path.is_dir() {
                if path.file_name().is_none_or(|n| n != "counterfeit") {
                    dirs.push(path);
                }
            } else if path.extension().is_some_and(|e| e == ext) {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}
