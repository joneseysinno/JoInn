//! Prim-specific readiness predicates.

mod build_parts;
mod choose_branch;

pub(crate) use build_parts::build_parts_ready;
pub(crate) use choose_branch::choose_branch_ready;
