//! `cargo xtask assay`.

mod agree;
mod run;

pub(crate) use agree::assay_agree;
pub(crate) use run::assay;
