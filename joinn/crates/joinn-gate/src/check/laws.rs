//! Check 2: every law holds on sampled inputs in the allele's frame.

mod contains_self;
mod degeneracy_advisory;

pub use degeneracy_advisory::degeneracy_advisory;

use crate::budget::Budget;
use crate::eval::{CellOracles, check_law};
use crate::oracle::Oracle;
use joinn_dna::Cell;
use joinn_frame::{FrameRef, FrameRegistry, Verdict};

/// Run every named law.
pub fn check(
    cell: &Cell,
    frames: &FrameRegistry,
    oracle: &dyn Oracle,
    cells: &CellOracles,
    allele_frame: FrameRef,
    budget: &Budget,
) -> Verdict<()> {
    for law in cell.coding.laws.values() {
        match check_law(law, frames, oracle, cells, allele_frame, budget) {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(())
}
