//! Formula evaluation over an oracle. Deterministic sampling and shrinking.

mod apply_oracle;
mod check_law;
mod eval_closed;
mod eval_forall;
mod eval_formula;
mod eval_predicate;
mod eval_term;
mod idle;
mod shrink_env;
mod values_eq;

pub use check_law::check_law;
pub use eval_predicate::eval_predicate;

use joinn_frame::Hash;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::oracle::Oracle;

/// Oracles for `cell:<hash>@…` terms.
pub type CellOracles = BTreeMap<Hash, Arc<dyn Oracle>>;

/// A passing law report.
#[derive(Clone, Debug)]
pub struct LawReport {
    /// Law name.
    pub name: String,
    /// How many samples were drawn.
    pub samples: u32,
}
