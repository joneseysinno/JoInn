//! V23 on the live engine.

use joinn_dna::{Body, Cell, NativeId};
use joinn_frame::{Hash, Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::refuse::refuse;
use super::{DnaFire, v23_execute};

/// V23 on the live engine: the DNA body must run (nonzero steps) and match the sealed oracle.
pub fn v23_execute_dna(
    dna: &dyn DnaFire,
    body: &Body,
    cell: &Cell,
    cells: &BTreeMap<Hash, Cell>,
    bodies: &BTreeMap<Hash, Body>,
    sealed: NativeId,
    native: &dyn Oracle,
    inputs: &BTreeMap<u32, Value>,
) -> Verdict<()> {
    match v23_execute(cell, sealed, native, inputs) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => return Verdict::Refused(r),
    }
    let want = match native.apply(inputs) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let got = match dna.fire(body, cell, cells, bodies, inputs) {
        Verdict::Ok(m) => m,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    if dna.last_steps() == 0 {
        return Verdict::Refused(refuse("V23: engine took zero steps"));
    }
    if want != got {
        return Verdict::Refused(refuse("V23: DNA body and sealed allele disagree"));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::v23_execute::v23_execute;
    use super::super::{fold, unfold};
    use crate::alleles::AddInt;
    use joinn_dna::{AlleleBody, NativeId, hash, sum_cell};
    use joinn_frame::Verdict;
    use std::collections::BTreeMap;

    #[test]
    fn v23_wrong_native_refuses() {
        let cell = sum_cell();
        let unfolded = unfold(&cell, NativeId("mul@ℤ".into()));
        match unfolded.alleles.first().map(|a| &a.body) {
            Some(AlleleBody::Native(id)) => assert_eq!(id.0, "mul@ℤ"),
            _ => panic!("expected native"),
        }
        match v23_execute(&cell, NativeId("mul@ℤ".into()), &AddInt, &BTreeMap::new()) {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("mul@ℤ") || r.reason.contains("unfolded"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(()) => panic!("wrong unfold must refuse"),
        }
        let h = hash(&cell.coding);
        let folded = fold(&cell, h);
        let bad = unfold(&folded, NativeId("mul@ℤ".into()));
        match bad.alleles.first().map(|a| &a.body) {
            Some(AlleleBody::Native(id)) if id.0 == "add@ℤ" => panic!("wrong unfold kept add@ℤ"),
            Some(AlleleBody::Native(id)) => assert_eq!(id.0, "mul@ℤ"),
            _ => panic!("expected native"),
        }
    }
}
