//! Admit a child cell only when its lineage names the parent.

use joinn_dna::{hash, Cell};
use joinn_frame::Verdict;
use joinn_gate::{Budget, Gate};

/// Refuse unless `proposed`'s lineage is the parent's coding hash, then admit.
pub(crate) fn admit_lineage(proposed: &Cell, parent: &Cell) -> Result<(), String> {
    let want = hash(&parent.coding);
    match proposed.coding.lineage {
        Some(h) if h == want => {}
        other => {
            return Err(format!(
                "lineage {:?} does not name parent {}",
                other.map(|h| h.to_hex()),
                want.to_hex()
            ));
        }
    }
    let gate = Gate::new(Budget::default(), joinn_prim::sealed_natives());
    match gate.admit_cell(proposed, Some(parent)) {
        Verdict::Ok(_) => Ok(()),
        Verdict::Refused(r) => Err(format!("refused: {}", r.reason)),
    }
}
