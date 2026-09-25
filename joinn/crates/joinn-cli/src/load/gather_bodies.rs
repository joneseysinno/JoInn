//! Gather corpus bodies into a store; a single face per coding hash.

use joinn_dna::{Cell, parse_body};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use joinn_link::BodyStore;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Load every `.body` under `corpus`, skipping `variants` and `controls` folders.
pub(crate) fn gather_bodies(
    corpus: &Path,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<BodyStore, String> {
    let frames = FrameRegistry::phase1();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut dirs = vec![corpus.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        let mut entries: Vec<_> = rd
            .map(|e| e.map_err(|err| err.to_string()))
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|e| e.file_name());
        for ent in entries {
            let path = ent.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "variants" || name == "controls" {
                    continue;
                }
                dirs.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) == Some("body") {
                files.push(path);
            }
        }
    }
    files.sort_by(|a, b| {
        a.file_name()
            .cmp(&b.file_name())
            .then_with(|| a.as_os_str().cmp(b.as_os_str()))
    });

    let mut store = BodyStore::new();
    for path in files {
        let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let Verdict::Ok(body) = parse_body(&src, &frames) else {
            continue;
        };
        let source = path.display().to_string();
        match store.insert(body, cells.clone(), &source) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    Ok(store)
}
