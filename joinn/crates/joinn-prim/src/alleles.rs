//! Native alleles for the Phase 1 cells. Arithmetic lives here, not in the gate.

mod add_int;
mod add_int_allele;
mod add_rat;
mod add_rat_allele;
mod canon_int;
mod canon_rat;
mod canon_text;
mod format_int;
mod int_of;
mod parse_text;
mod rat_of;
mod refuse;
mod two_in;

pub use add_int::AddInt;
pub use add_int_allele::add_int_allele;
pub use add_rat::AddRat;
pub use add_rat_allele::add_rat_allele;
pub use format_int::FormatInt;
pub use parse_text::ParseText;

pub(crate) use canon_int::canon_int;
#[cfg(any(test, feature = "mutants"))]
pub(crate) use canon_rat::canon_rat;
pub(crate) use int_of::int_of;
#[cfg(any(test, feature = "mutants"))]
pub(crate) use rat_of::rat_of;
