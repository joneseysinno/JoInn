//! Frames, values, verdicts, and the obligation harness.
//!
//! This crate knows nothing about DNA, laws, contracts, or the gate.

#![forbid(unsafe_code)]

pub mod canon;
pub mod conformance;
pub mod frame;
pub mod frames;
pub mod hash;
pub mod registry;
pub mod rng;
pub mod term;
pub mod value;
pub mod verdict;

pub use canon::{CanonWriter, nfc};
pub use frame::{Case, Frame, FrameId, FrameRef, OpName, Signature};
pub use frames::{DEFAULT_SIZE, IntFrame, RatFrame, TextFrame};
pub use hash::{
    Hash, TAG_ALLELE, TAG_BODY, TAG_CELL, TAG_DESCRIPTION, TAG_UNIVERSE, TAG_WITNESS, keyed_hash,
};
pub use registry::FrameRegistry;
pub use rng::SeedRng;
pub use term::Term;
pub use value::Value;
pub use verdict::{CheckId, CounterExample, Refusal, Subject, Verdict};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_has_no_public_constructor() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/value_fields.rs");
    }

    #[test]
    fn hash_is_stable_and_tagged() {
        let a = keyed_hash(TAG_CELL, b"hello\n");
        let b = keyed_hash(TAG_CELL, b"hello\n");
        let c = keyed_hash(TAG_ALLELE, b"hello\n");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.to_hex().len(), 64);
    }
}
