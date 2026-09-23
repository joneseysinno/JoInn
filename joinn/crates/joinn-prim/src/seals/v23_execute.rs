//! V23 as fold/unfold with the named native.

use joinn_dna::{AlleleBody, Cell, NativeId, hash};
use joinn_frame::{Value, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::fold::fold;
use super::refuse::refuse;
use super::unfold::unfold;

/// V23 as fold/unfold with the named native. Engine execution lives with `DnaFire`.
pub fn v23_execute(
    cell: &Cell,
    sealed: NativeId,
    native: &dyn Oracle,
    inputs: &BTreeMap<u32, Value>,
) -> Verdict<()> {
    let h = hash(&cell.coding);
    let folded = fold(cell, h);
    let unfolded = unfold(&folded, sealed.clone());
    if hash(&unfolded.coding) != h || hash(&folded.coding) != h {
        return Verdict::Refused(refuse("V23: fold/unfold moved the coding hash"));
    }
    match unfolded.alleles.first().map(|a| &a.body) {
        Some(AlleleBody::Native(id)) if id.0 == sealed.0 => {}
        Some(AlleleBody::Native(id)) => {
            return Verdict::Refused(refuse(&format!(
                "V23: unfolded to {} not {}",
                id.0, sealed.0
            )));
        }
        _ => return Verdict::Refused(refuse("V23: unfolded cell has no native")),
    }
    let original = match cell.alleles.first().map(|a| &a.body) {
        Some(AlleleBody::Native(id)) => id.0.clone(),
        _ => sealed.0.clone(),
    };
    if original != sealed.0 {
        return Verdict::Refused(refuse(&format!(
            "V23: unfolded to {} not {original}",
            sealed.0
        )));
    }
    match native.apply(inputs) {
        Verdict::Ok(_) => Verdict::Ok(()),
        Verdict::Refused(r) => Verdict::Refused(r),
    }
}

#[cfg(test)]
mod tests {
    use super::v23_execute;
    use crate::alleles::AddInt;
    use joinn_dna::{NativeId, sum_cell};
    use joinn_frame::{Frame, IntFrame, Term, Verdict};
    use std::collections::BTreeMap;

    #[test]
    fn v23_execute_on_sum() {
        let cell = sum_cell();
        let a = match IntFrame::new().canonicalize(Term::int(2)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let b = match IntFrame::new().canonicalize(Term::int(3)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match v23_execute(
            &cell,
            NativeId("add@ℤ".into()),
            &AddInt,
            &BTreeMap::from([(0, a), (1, b)]),
        ) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
