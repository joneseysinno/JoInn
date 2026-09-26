//! The assay report does not move under phenotype, alleles, lenses, or renames.

mod edit_one;
mod phenotype;
mod run;

pub(crate) use run::assay_invariance;
