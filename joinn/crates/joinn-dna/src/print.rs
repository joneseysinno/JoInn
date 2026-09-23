//! Canonical printer. Deterministic member order, NFC, LF, one trailing newline.

mod print_allele;
mod print_args;
mod print_coding;
mod print_contract;
mod print_formula;
mod print_term_;
mod print_witness;

pub use print_allele::print_allele;
pub use print_coding::print_coding;
pub use print_formula::print_formula;
pub use print_term_::print_term_;
