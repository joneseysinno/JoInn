//! `Genotype` seal: only coding-region types can be hashed.

mod allele;
mod body_coding;
mod coding_region;
mod hash_fn;

pub use hash_fn::hash;

use crate::body::BodyCoding;
use crate::model::{Allele, CodingRegion};

mod sealed {
    /// Implemented only for coding-region types, only inside this module.
    pub trait Sealed {}
}

/// Types that may be content-addressed. `RegulatoryRegion` does not implement this.
pub trait Genotype: sealed::Sealed {
    /// Domain tag.
    fn domain_tag() -> &'static [u8];
    /// Write canonical bytes.
    fn encode(&self, w: &mut joinn_frame::CanonWriter);
}

impl sealed::Sealed for CodingRegion {}
impl sealed::Sealed for Allele {}
impl sealed::Sealed for BodyCoding {}
