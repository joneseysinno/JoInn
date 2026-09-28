//! Every admitted corpus cell and every corpus body outside counterfeit/, loaded once.

use joinn_dna::{Cell, hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::BodyStore;
use std::collections::BTreeMap;
use std::fs;
use std::sync::OnceLock;

use super::walk_cells::walk_cells;
use super::workspace_root::workspace_root;

type Loaded = (BTreeMap<Hash, Cell>, BodyStore);

static STORE: OnceLock<Result<Loaded, String>> = OnceLock::new();

/// Cells keyed by hash, and a store holding every corpus body that parses.
pub(crate) fn corpus_store() -> Result<&'static Loaded, String> {
    STORE
        .get_or_init(|| {
            let root = workspace_root()?;
            let frames = FrameRegistry::phase1();
            let corpus = root.join("corpus");
            let mut cell_paths = Vec::new();
            walk_cells(&corpus, &mut cell_paths)?;
            let mut cells = BTreeMap::new();
            for path in &cell_paths {
                let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
                if let Verdict::Ok(cell) = parse_cell(&text, &frames) {
                    cells.entry(hash(&cell.coding)).or_insert(cell);
                }
            }
            let mut dirs = vec![corpus];
            let mut body_paths = Vec::new();
            while let Some(dir) = dirs.pop() {
                let mut names = Vec::new();
                for ent in fs::read_dir(&dir).map_err(|e| e.to_string())? {
                    names.push(ent.map_err(|e| e.to_string())?.path());
                }
                names.sort();
                for name in names {
                    if name.is_dir() {
                        if name.file_name().is_some_and(|n| n == "counterfeit") {
                            continue;
                        }
                        dirs.push(name);
                    } else if name.extension().is_some_and(|e| e == "body") {
                        body_paths.push(name);
                    }
                }
            }
            body_paths.sort();
            let mut store = BodyStore::new();
            for path in &body_paths {
                let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
                let Verdict::Ok(body) = parse_body(&text, &frames) else {
                    continue;
                };
                match store.insert(body, cells.clone(), &path.display().to_string()) {
                    Verdict::Ok(_) => {}
                    Verdict::Refused(r) if r.reason.contains("distinct faces") => {}
                    Verdict::Refused(r) => return Err(r.reason),
                }
            }
            Ok((cells, store))
        })
        .as_ref()
        .map_err(String::clone)
}
