//! Sealed alleles plus the mutant register. Test and xtask only.

use crate::mutants::{
    Difference, Impostor, Leading, Max, NoRestrict, PanicNeg, Plus1, Saturating, Times,
    TurnPosOnly, Wrapping, WrongFrame, WrongRat, Zero,
};
use joinn_gate::NativeRegistry;
use std::sync::Arc;

use super::sealed_natives::sealed_natives;

/// Sealed alleles plus the mutant register. Test and xtask only.
pub fn natives_with_mutants() -> NativeRegistry {
    // allow(vocab): mutant register, cfg-gated
    let mut r = sealed_natives();
    r.insert("mutant.plus1", Arc::new(Plus1));
    r.insert("mutant.difference", Arc::new(Difference));
    r.insert("mutant.times", Arc::new(Times));
    r.insert("mutant.max", Arc::new(Max));
    r.insert("mutant.saturating", Arc::new(Saturating));
    r.insert("mutant.impostor", Arc::new(Impostor));
    r.insert("mutant.zero", Arc::new(Zero));
    r.insert("mutant.wrong_frame", Arc::new(WrongFrame));
    r.insert("mutant.panic_neg", Arc::new(PanicNeg));
    r.insert("mutant.wrong_rat", Arc::new(WrongRat));
    r.insert("mutant.no_restrict", Arc::new(NoRestrict));
    r.insert("mutant.leading", Arc::new(Leading));
    r.insert("mutant.wrapping", Arc::new(Wrapping));
    r.insert("mutant.turn_pos_only", Arc::new(TurnPosOnly));
    r
}
