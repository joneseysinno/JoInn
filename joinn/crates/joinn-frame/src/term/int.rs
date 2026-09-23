//! `Term::int`.

use super::Term;
use num_bigint::BigInt;

impl Term {
    /// Integer term.
    pub fn int(n: impl Into<BigInt>) -> Self {
        Term::Int(n.into())
    }
}
