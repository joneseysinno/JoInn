//! Gather corpus bodies by coding hash; a single face per hash.

use joinn_dna::{hash, parse_body, Body, Cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Load every `.body` under `corpus`, skipping `variants` and `controls` folders.
/// Refuses another body with the same coding hash and a different regulatory region.
pub(crate) fn gather_bodies(
    corpus: &Path,
    cells: &BTreeMap<Hash, Cell>,
) -> Result<BTreeMap<Hash, (Body, BTreeMap<Hash, Cell>)>, String> {
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

    let mut by_hash: BTreeMap<Hash, (Body, BTreeMap<Hash, Cell>, PathBuf)> = BTreeMap::new();
    for path in files {
        let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let Verdict::Ok(body) = parse_body(&src, &frames) else {
            continue;
        };
        let id = hash(&body.coding);
        if let Some((prev, _, prev_path)) = by_hash.get(&id) {
            if prev.regulatory != body.regulatory {
                return Err(format!(
                    "coding hash has distinct faces: {} and {}",
                    prev_path.display(),
                    path.display()
                ));
            }
            continue;
        }
        by_hash.insert(id, (body, cells.clone(), path));
    }
    Ok(by_hash
        .into_iter()
        .map(|(h, (b, c, _))| (h, (b, c)))
        .collect())
}
