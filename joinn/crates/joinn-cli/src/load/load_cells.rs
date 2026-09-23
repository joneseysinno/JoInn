//! Load every corpus cell the parser accepts.

use joinn_dna::{hash, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Load every `.cell` under `corpus` that parses. A refusal is skipped:
/// the counterfeit corpus is refused on purpose.
pub(crate) fn load_cells(corpus: &Path) -> Result<BTreeMap<Hash, joinn_dna::Cell>, String> {
    let frames = FrameRegistry::phase1();
    let mut cells: BTreeMap<Hash, joinn_dna::Cell> = BTreeMap::new();
    let mut dirs = vec![corpus.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        for ent in rd {
            let ent = ent.map_err(|e| e.to_string())?;
            let path = ent.path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("cell") {
                continue;
            }
            let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            match parse_cell(&src, &frames) {
                Verdict::Ok(cell) => {
                    let id = hash(&cell.coding);
                    let replace = match cells.get(&id) {
                        Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
                        None => true,
                    };
                    if replace {
                        cells.insert(id, cell);
                    }
                }
                Verdict::Refused(_) => {}
            }
        }
    }
    Ok(cells)
}
