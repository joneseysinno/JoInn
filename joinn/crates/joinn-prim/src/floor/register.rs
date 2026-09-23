//! Every floor member. The count is even by arithmetic of opposition.

use super::Reference;
use super::bind::Bind;
use super::bound::Bound;
use super::build_prim::BuildPrim;
use super::case_prim::CasePrim;
use super::choose::Choose;
use super::eq_prim::EqPrim;
use super::fan::Fan;
use super::fill::Fill;
use super::grant::Grant;
use super::hash_prim::HashPrim;
use super::join::Join;
use super::pair::Pair;
use super::resolve_prim::ResolvePrim;
use super::revoke::Revoke;
use super::split::Split;
use super::unbind::Unbind;

/// Every floor member. The count is even by arithmetic of opposition.
pub fn register() -> Vec<Box<dyn Reference>> {
    vec![
        Box::new(EqPrim),
        Box::new(Choose),
        Box::new(Pair),
        Box::new(Split),
        Box::new(BuildPrim),
        Box::new(CasePrim),
        Box::new(HashPrim),
        Box::new(ResolvePrim),
        Box::new(Bound),
        Box::new(Fill),
        Box::new(Bind),
        Box::new(Unbind),
        Box::new(Grant),
        Box::new(Revoke),
        Box::new(Join),
        Box::new(Fan),
    ]
}
