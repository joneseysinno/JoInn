//! Declarations on a body or a universe. Phase 4 admits one sentence.

mod parse_assertions;

pub use parse_assertions::parse_assertions;

/// One declared sentence. A declaration refuses at admission; an assay never does.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Assertion {
    /// `assert H₁ = 0`: every loop is filled.
    H1Zero,
}

impl Assertion {
    /// The sentence exactly as it is written and printed.
    pub fn sentence(self) -> &'static str {
        match self {
            Assertion::H1Zero => "assert H₁ = 0",
        }
    }
}
