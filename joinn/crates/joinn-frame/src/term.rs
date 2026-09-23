//! The only term shapes in Phase 1. A frame gives them meaning.

mod int;
mod text;

use num_bigint::BigInt;

/// A frame-agnostic term. Values wrap a term in a frame.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Term {
    /// An integer coefficient or an ℤ payload.
    Int(BigInt),
    /// A Unicode string. Stored NFC after canonicalization.
    Text(String),
    /// A sequence of terms. ℚ is `[numer, denom]`.
    Seq(Vec<Term>),
}
