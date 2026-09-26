//! `cargo xtask assay`.

mod agree;
mod invariance;
mod run;

pub(crate) use agree::assay_agree;
pub(crate) use invariance::assay_invariance;
pub(crate) use run::assay;
