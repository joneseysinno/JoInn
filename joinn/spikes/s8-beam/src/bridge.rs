//! The only way across the mirror: a moment becomes a curvature through E·I,
//! read from a pinned edition. Every read is counted.

mod curvature;
mod new;
mod parse;
mod pinned;
#[cfg(test)]
mod testing;

pub use parse::parse;
pub use pinned::pinned;

use num_rational::BigRational;

/// The testimony the spike reads: one material's E and one section's Ix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edition {
    source: String,
    material: String,
    /// ksi (kip/in²).
    e: BigRational,
    shape: String,
    /// in⁴.
    ix: BigRational,
}

/// A bridge holds an edition or none, and counts every read of it.
#[derive(Clone, Debug)]
pub struct Bridge {
    edition: Option<Edition>,
    reads: u32,
}

impl Edition {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn material(&self) -> &str {
        &self.material
    }

    pub fn e(&self) -> &BigRational {
        &self.e
    }

    pub fn shape(&self) -> &str {
        &self.shape
    }

    pub fn ix(&self) -> &BigRational {
        &self.ix
    }
}

impl Bridge {
    pub fn reads(&self) -> u32 {
        self.reads
    }
}
