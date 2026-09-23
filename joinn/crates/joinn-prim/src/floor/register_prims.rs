//! Insert matter floor members into a native register under `prim:<name>`.

use std::sync::Arc;

use super::build_prim::BuildPrim;
use super::case_prim::CasePrim;
use super::choose::Choose;
use super::eq_prim::EqPrim;
use super::pair::Pair;
use super::split::Split;

/// Insert matter floor members into a native register under `prim:<name>`.
/// Physics members stay available to the engine; they are not allele-visible.
pub fn register_prims(natives: &mut joinn_gate::NativeRegistry) {
    natives.insert("prim:eq", Arc::new(EqPrim));
    natives.insert("prim:choose", Arc::new(Choose));
    natives.insert("prim:pair", Arc::new(Pair));
    natives.insert("prim:split", Arc::new(Split));
    natives.insert("prim:build", Arc::new(BuildPrim));
    natives.insert("prim:case", Arc::new(CasePrim));
}
