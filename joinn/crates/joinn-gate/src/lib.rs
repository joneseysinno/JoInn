//! Law evaluation, sampling, witness replay, the four checks, refusals.

#![forbid(unsafe_code)]

pub mod budget;
pub mod check;
pub mod demos;
pub mod eval;
pub mod gate;
pub mod natives;
pub mod oracle;
pub mod testimony;

pub use budget::Budget;
pub use eval::{CellOracles, eval_predicate};
pub use gate::{Accepted, Artifact, Gate, GateItem, run_opposed};
pub use natives::NativeRegistry;
pub use oracle::Oracle;
pub use testimony::TestimonyStore;

#[cfg(test)]
mod tests {
    use super::*;
    use joinn_dna::{hash, sum_cell};

    #[test]
    fn thousand_testimony_appends_leave_hash_unchanged() {
        let cell = sum_cell();
        let before = hash(&cell.coding);
        let mut store = TestimonyStore::memory();
        for i in 0..1000 {
            store.append(before, &format!("n={i}"));
        }
        assert_eq!(hash(&cell.coding), before);
    }
}
