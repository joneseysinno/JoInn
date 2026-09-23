//! Convenience: the ℤ add allele on `Sum`.

use joinn_dna::{Allele, AlleleBody, NativeId};
use joinn_frame::FrameRef;

/// Convenience: the ℤ add allele on `Sum`.
pub fn add_int_allele() -> Allele {
    Allele {
        frame: FrameRef::int(),
        body: AlleleBody::Native(NativeId("add@ℤ".into())),
        witnesses: Vec::new(),
    }
}
