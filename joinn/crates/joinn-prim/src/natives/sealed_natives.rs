//! Sealed alleles for the Phase 1 cells, plus matter floor members.

use crate::alleles::{AddInt, AddRat, FormatInt, ParseText};
use crate::floor::register_prims;
use crate::seals::MulIntSealed;
use crate::turns::{AddIntTurn0, AddIntTurn1};
use joinn_gate::NativeRegistry;
use std::sync::Arc;

/// Sealed alleles for the Phase 1 cells, plus matter floor members.
pub fn sealed_natives() -> NativeRegistry {
    let mut r = NativeRegistry::new();
    r.insert("add@ℤ", Arc::new(AddInt));
    r.insert("add@ℚ", Arc::new(AddRat));
    r.insert("parse@Text", Arc::new(ParseText));
    r.insert("format@ℤ", Arc::new(FormatInt));
    r.insert("mul@ℤ", Arc::new(MulIntSealed));
    r.insert("add@ℤ.turn0", Arc::new(AddIntTurn0));
    r.insert("add@ℤ.turn1", Arc::new(AddIntTurn1));
    register_prims(&mut r);
    r
}
