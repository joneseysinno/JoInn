//! Every corpus cell that parses, keyed by its coding hash.

use joinn_dna::{Cell, hash, parse_cell};
use joinn_frame::{FrameRegistry, Hash, Verdict};
use std::collections::BTreeMap;
use std::fs;

use crate::fns::{walk_cells, workspace_root};

/// Cells that are refused on parse (fixtures that must be refused) are left out;
/// the register names its cells by hash, so a missing one is refused by name.
/// Files sharing a coding region (`sum`, `sum_b`, …) pool their alleles.
pub(crate) fn corpus_cells(frames: &FrameRegistry) -> Result<BTreeMap<Hash, Cell>, String> {
    let mut paths = Vec::new();
    walk_cells(&workspace_root()?.join("corpus"), &mut paths)?;
    let mut cells: BTreeMap<Hash, Cell> = BTreeMap::new();
    for path in paths {
        let src = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let Verdict::Ok(cell) = parse_cell(&src, frames) else {
            continue;
        };
        match cells.get_mut(&hash(&cell.coding)) {
            Some(kept) => {
                for allele in cell.alleles {
                    if !kept.alleles.contains(&allele) {
                        kept.alleles.push(allele);
                    }
                }
            }
            None => {
                cells.insert(hash(&cell.coding), cell);
            }
        }
    }
    Ok(cells)
}
