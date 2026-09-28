//! Every corpus `.body`, bound alone against every corpus cell, in path order.

use joinn_dna::{Body, Cell, hash, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;

use super::lone_body::lone_body;
use crate::fns::walk_cells::walk_cells;
use crate::fns::workspace_root::workspace_root;

/// A corpus-relative path with the lone body it binds to, or the refusal.
pub(crate) type BoundBody = (String, Verdict<(Body, BTreeMap<Hash, Cell>)>);

/// `counterfeit/` directories are skipped, as `walk_cells` and the assay skip them.
pub(crate) fn corpus_bodies() -> Result<Vec<BoundBody>, String> {
    let corpus = workspace_root()?.join("corpus");
    let frames = FrameRegistry::phase1();
    let mut cell_paths = Vec::new();
    walk_cells(&corpus, &mut cell_paths)?;
    let mut cells = BTreeMap::new();
    for path in &cell_paths {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if let Verdict::Ok(cell) = parse_cell(&text, &frames) {
            cells.entry(hash(&cell.coding)).or_insert(cell);
        }
    }
    let mut dirs = vec![corpus.clone()];
    let mut paths = Vec::new();
    while let Some(dir) = dirs.pop() {
        let mut names = Vec::new();
        for ent in fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            names.push(ent.map_err(|e| e.to_string())?.path());
        }
        names.sort();
        for name in names {
            if name.is_dir() {
                if name.file_name().is_none_or(|n| n != "counterfeit") {
                    dirs.push(name);
                }
            } else if name.extension().is_some_and(|e| e == "body") {
                paths.push(name);
            }
        }
    }
    paths.sort();
    let mut out = Vec::new();
    for path in paths {
        let rel = match path.strip_prefix(&corpus) {
            Ok(rel) => rel
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join("/"),
            Err(_) => path.display().to_string(),
        };
        let text = fs::read_to_string(&path).map_err(|e| format!("{rel}: {e}"))?;
        let bound = lone_body(&text, &rel, &cells);
        out.push((rel, bound));
    }
    Ok(out)
}
