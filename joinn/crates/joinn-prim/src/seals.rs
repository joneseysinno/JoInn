//! Seals, reference bodies, fold/unfold, agreement.

use joinn_dna::NativeId;
use joinn_frame::Hash;
use std::num::NonZeroU32;

mod agree;
mod agree_injected_disagreement;
mod agree_one;
mod bounded_int;
mod bounded_rat;
mod bounded_text;
mod check_seal_dag;
mod check_v33;
mod check_v33_full;
mod drive_admits;
mod drive_frame;
mod find_cell_for_native;
mod fold;
mod mul_int_sealed;
mod mutant_sealed_as_reference_body;
mod out_port;
mod refuse;
mod refuse_seal;
mod register_seal;
mod round_trip;
mod sample_inputs;
mod sample_port;
mod seal_register;
mod sealed_oracle;
mod truth_violation;
mod unfold;
mod v23_execute;
mod v23_execute_dna;
mod visit_seal_node;
#[cfg(any(test, feature = "mutants"))]
mod wrapping_caught_by_agree;

pub use agree::agree;
pub use agree_injected_disagreement::agree_injected_disagreement;
pub use agree_one::agree_one;
pub use check_seal_dag::check_seal_dag;
pub use check_v33::check_v33;
pub use check_v33_full::check_v33_full;
pub use drive_admits::drive_admits;
pub use find_cell_for_native::find_cell_for_native;
pub use fold::fold;
pub use mul_int_sealed::MulIntSealed;
pub use mutant_sealed_as_reference_body::mutant_sealed_as_reference_body;
pub use register_seal::register_seal;
pub use seal_register::seal_register;
pub use unfold::unfold;
pub use v23_execute::v23_execute;
pub use v23_execute_dna::v23_execute_dna;
#[cfg(any(test, feature = "mutants"))]
pub use wrapping_caught_by_agree::wrapping_caught_by_agree;

/// Sample count `cargo xtask agree` and gate item 1 both read.
pub const AGREE_SAMPLES: u32 = 256;
/// `parse(format n) = n` sample count.
pub const ROUNDTRIP_SAMPLES: u32 = 10_000;

/// A body identity. A reference allele is a body.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BodyRef {
    /// Coding-region hash of the reference body.
    pub hash: Hash,
}

/// Which port bounds the reference's loop, and the bound.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Drive {
    /// Port index.
    pub port: u32,
    /// Inclusive magnitude bound for that port. A bound that admits one value is not a bound.
    pub bound: NonZeroU32,
}

impl Drive {
    /// Construct a drive. `bound` is already nonzero.
    pub const fn new(port: u32, bound: NonZeroU32) -> Self {
        Self { port, bound }
    }
}

/// A seal: one cell, a reference body, a counterfeit, a sealed native.
#[derive(Clone, Debug)]
pub struct Seal {
    /// The cell's coding-region hash. Found by lookup, not assigned.
    pub cell: Hash,
    /// Reference body. Must agree on every sample.
    pub reference: BodyRef,
    /// Counterfeit body. Must disagree on at least one sample.
    pub counterfeit: BodyRef,
    /// Fast path.
    pub sealed: NativeId,
    /// Drive declaration. Total.
    pub drives: Drive,
    /// Declared hole: format ∘ parse is one-way. Data, not a function returning true.
    pub one_way: bool,
}

/// Seal table. Bodies are filled by the caller (corpus / live). No IO here.
#[derive(Clone, Copy, Debug)]
pub struct SealSpec {
    /// Reference body file stem under `corpus/phase21` or `corpus/phase22`.
    pub reference_file: &'static str,
    /// Counterfeit body file stem under `corpus/phase22/counterfeit`.
    pub counterfeit_file: &'static str,
    /// Sealed native.
    pub sealed: &'static str,
    /// Drive port.
    pub drive_port: u32,
    /// Drive bound (nonzero).
    pub drive_bound: u32,
    /// Declared one-way hole.
    pub one_way: bool,
}

/// Fire a DNA body on the live engine. Implemented in `joinn-live`.
pub trait DnaFire {
    /// Run `body` as the allele of `cell` with `inputs`.
    fn fire(
        &self,
        body: &joinn_dna::Body,
        cell: &joinn_dna::Cell,
        cells: &std::collections::BTreeMap<Hash, joinn_dna::Cell>,
        bodies: &std::collections::BTreeMap<Hash, joinn_dna::Body>,
        inputs: &std::collections::BTreeMap<u32, joinn_frame::Value>,
    ) -> joinn_frame::Verdict<std::collections::BTreeMap<u32, joinn_frame::Value>>;

    /// Steps taken by the last `fire`, if the engine records them.
    fn last_steps(&self) -> u64 {
        0
    }
}
