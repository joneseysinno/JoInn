//! `cargo xtask forces`: the force register is admitted (order-blind and opposed),
//! and its plants are refused or reported as §2.2 says.

mod corpus_cells;
mod run;

pub(crate) use corpus_cells::corpus_cells;
pub(crate) use run::forces;
