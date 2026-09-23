//! Degeneracy advisory.

use joinn_dna::{Cell, Formula, Term_};

use super::contains_self::contains_self;

/// Degeneracy advisory: laws determine `self` pointwise on the sampled domain.
/// Advisory, never a refusal of admission.
pub fn degeneracy_advisory(cell: &Cell) -> Option<String> {
    for law in cell.coding.laws.values() {
        if let Formula::ForAll { body, .. } = &law.formula {
            if let Formula::Eq(Term_::SelfAt { .. }, rhs) = body.as_ref() {
                if !contains_self(rhs) {
                    if let Term_::FrameOp { .. } = rhs {
                        return Some(format!(
                            "degeneracy advisory: law {} pins self to a frame op",
                            law.name.0
                        ));
                    }
                }
            }
        }
    }
    None
}
