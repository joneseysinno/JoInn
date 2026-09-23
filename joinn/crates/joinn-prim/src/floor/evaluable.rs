//! Evaluable matter + physics lookup members.

use super::Reference;
use super::build_prim::BuildPrim;
use super::case_prim::CasePrim;
use super::choose::Choose;
use super::eq_prim::EqPrim;
use super::hash_prim::HashPrim;
use super::pair::Pair;
use super::resolve_prim::ResolvePrim;
use super::split::Split;

/// Evaluable matter + physics lookup members. Space and the rest of physics are the engine.
pub(in crate::floor) fn evaluable() -> Vec<Box<dyn Reference>> {
    vec![
        Box::new(EqPrim),
        Box::new(Choose),
        Box::new(Pair),
        Box::new(Split),
        Box::new(BuildPrim),
        Box::new(CasePrim),
        Box::new(HashPrim),
        Box::new(ResolvePrim),
    ]
}
