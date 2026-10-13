//! What a protein's partner reading is.

use super::Protein;
use crate::meaning::AminoAcid;

impl Protein {
    /// The opposites in reverse order (D19): what the partner strand reads.
    pub fn opposite(&self) -> Vec<AminoAcid> {
        self.acids.iter().rev().map(|a| a.opposite()).collect()
    }
}
