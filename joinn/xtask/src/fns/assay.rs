//! `cargo xtask assay`.

mod agree;
mod all;
mod invariance;
mod run;

pub(crate) use agree::assay_agree;
pub(crate) use all::assay_all;
pub(crate) use invariance::assay_invariance;
pub(crate) use run::assay;
