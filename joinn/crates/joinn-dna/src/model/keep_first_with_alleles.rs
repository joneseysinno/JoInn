//! One coding region, many files: which cell a cell map keeps.

use joinn_frame::Hash;
use std::collections::BTreeMap;

use super::Cell;

/// Inserts `cell` at `hash` when the map holds nothing there, or when the cell
/// it holds carries no allele and `cell` carries one. Otherwise the first cell
/// stays: a cell with alleles is never displaced.
pub fn keep_first_with_alleles(map: &mut BTreeMap<Hash, Cell>, hash: Hash, cell: Cell) {
    let replace = match map.get(&hash) {
        Some(prev) => prev.alleles.is_empty() && !cell.alleles.is_empty(),
        None => true,
    };
    if replace {
        map.insert(hash, cell);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::keep_first_with_alleles;
    use crate::{Cell, hash, sum_cell};

    /// The sum cell with its alleles, the same with none, and a second cell
    /// with alleles told apart from the first by its regulatory label.
    fn cells() -> (Cell, Cell, Cell) {
        let with = sum_cell();
        let mut without = with.clone();
        without.alleles.clear();
        let mut other = with.clone();
        other.regulatory.labels.insert(2, "total".into());
        (with, without, other)
    }

    fn label(map: &BTreeMap<joinn_frame::Hash, Cell>) -> Option<(usize, String)> {
        map.values().next().map(|c| {
            (
                c.alleles.len(),
                c.regulatory.labels.get(&2).cloned().unwrap_or_default(),
            )
        })
    }

    #[test]
    fn an_empty_map_takes_the_cell() {
        let (with, without, _) = cells();
        let id = hash(&with.coding);
        let mut map = BTreeMap::new();
        keep_first_with_alleles(&mut map, id, without);
        assert_eq!(label(&map), Some((0, "sum".into())));
    }

    #[test]
    fn a_cell_with_alleles_displaces_one_without() {
        let (with, without, _) = cells();
        let id = hash(&with.coding);
        let mut map = BTreeMap::new();
        keep_first_with_alleles(&mut map, id, without);
        keep_first_with_alleles(&mut map, id, with);
        assert_eq!(label(&map), Some((1, "sum".into())));
    }

    #[test]
    fn a_cell_without_alleles_never_displaces_one_with() {
        let (with, without, _) = cells();
        let id = hash(&with.coding);
        let mut map = BTreeMap::new();
        keep_first_with_alleles(&mut map, id, with);
        keep_first_with_alleles(&mut map, id, without);
        assert_eq!(label(&map), Some((1, "sum".into())));
    }

    #[test]
    fn when_both_carry_alleles_the_first_is_kept() {
        let (with, _, other) = cells();
        let id = hash(&with.coding);
        assert_eq!(id, hash(&other.coding), "one coding region");
        let mut map = BTreeMap::new();
        keep_first_with_alleles(&mut map, id, with);
        keep_first_with_alleles(&mut map, id, other);
        assert_eq!(label(&map), Some((1, "sum".into())));
    }
}
