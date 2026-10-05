//! Every corpus body a universe can bind by hash.

use joinn_dna::{Cell, parse_body};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::BodyStore;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Each `.body` under `corpus` that parses, outside `counterfeit/`, in path
/// order. A body whose coding is already held under another file is skipped.
pub(crate) fn load_store(corpus: &Path, cells: &BTreeMap<Hash, Cell>) -> Result<BodyStore, String> {
    let frames = FrameRegistry::phase1();
    let mut paths = Vec::new();
    let mut dirs = vec![corpus.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        for ent in rd {
            let path = ent.map_err(|e| e.to_string())?.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n != "counterfeit") {
                    dirs.push(path);
                }
            } else if path.extension().is_some_and(|e| e == "body") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    let mut store = BodyStore::new();
    for path in &paths {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let Verdict::Ok(body) = parse_body(&text, &frames) else {
            continue;
        };
        match store.insert(body, cells.clone(), &path.display().to_string()) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    Ok(store)
}
