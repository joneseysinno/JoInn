//! Native registries for sealed alleles and engine services.

mod engine_natives;
#[cfg(any(test, feature = "mutants"))]
mod natives_with_mutants; // allow(vocab): cfg-gated mutant register module
mod sealed_natives;

pub use engine_natives::engine_natives;
#[cfg(any(test, feature = "mutants"))]
pub use natives_with_mutants::natives_with_mutants; // allow(vocab): cfg-gated mutant register re-export
pub use sealed_natives::sealed_natives;
