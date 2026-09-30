//! Every corpus cell that parses, keyed by its coding hash.

use joinn_dna::{Cell, hash, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::{walk_cells, workspace_root};

/// Cells that are refused on parse (fixtures that must be refused) are left out;
/// the register names its cells by hash, so a missing one is refused by name.
/// Files sharing a coding region (`sum`, `sum_b`, …) keep the corpus's one rule:
/// the first file that carries alleles.
pub(crate) fn corpus_cells(frames: &FrameRegistry) -> Result<BTreeMap<Hash, Cell>, String> {
    let mut paths = Vec::new();
    walk_cells(&workspace_root()?.join("corpus"), &mut paths)?;
    let mut cells: BTreeMap<Hash, Cell> = BTreeMap::new();
    for path in paths {
        let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let Verdict::Ok(cell) = parse_cell(&src, frames) else {
            continue;
        };
        let id = hash(&cell.coding);
        let replace = match cells.get(&id) {
            Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
            None => true,
        };
        if replace {
            cells.insert(id, cell);
        }
    }
    Ok(cells)
}
