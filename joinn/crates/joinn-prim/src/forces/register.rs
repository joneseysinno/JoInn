//! The register's rows. One: combine on ℤ 1, answered by the sum cell's coding
//! region and opposed by `sum_turn`.

use joinn_frame::Hash;
use std::sync::LazyLock;

use super::{ForceKind, RegisterRow};

/// The sum cell: ℤ's response to combine. Its laws are identity, commutative
/// and associative; its allele is `add@ℤ`.
const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";

/// `sum_turn`: lineage the sum cell, `turn 0 from {1 2}`.
const SUM_TURN: &str = "6fcb1e7397454543fad3b22d4b1c310932985e44d1ed64ae94142ab82339e9cf";

static ROWS: LazyLock<[RegisterRow; 1]> = LazyLock::new(|| {
    let zero = Hash::from_bytes([0; 32]);
    [RegisterRow {
        force: ForceKind::Combine,
        frame: ("ℤ", 1),
        response: Hash::parse_hex(SUM).unwrap_or(zero),
        separate: Hash::parse_hex(SUM_TURN).unwrap_or(zero),
    }]
});

/// Every row. A digest that fails to parse becomes the zero hash, which
/// `check_register` refuses as a cell that was not supplied.
pub fn register() -> &'static [RegisterRow] {
    ROWS.as_slice()
}

#[cfg(test)]
mod tests {
    use super::register;
    use crate::forces::ForceKind;

    #[test]
    fn the_one_row_is_combine_on_int_by_sum_opposed_by_sum_turn() {
        let rows = register();
        assert_eq!(rows.len(), 1);
        let row = rows[0];
        assert_eq!(row.force, ForceKind::Combine);
        assert_eq!(row.frame, ("ℤ", 1));
        assert_eq!(
            row.response.to_hex(),
            "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39"
        );
        assert_eq!(
            row.separate.to_hex(),
            "6fcb1e7397454543fad3b22d4b1c310932985e44d1ed64ae94142ab82339e9cf"
        );
    }
}
