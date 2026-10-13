//! The organelles an amino acid uses.

use super::Organelle;
use crate::meaning::AminoAcid;

impl AminoAcid {
    /// The world acids (sense, show, keep, recall) name at least one; the
    /// others name none.
    pub fn organelles(self) -> &'static [Organelle] {
        match self {
            AminoAcid::Show => &[Organelle::Screen],
            AminoAcid::Sense => &[Organelle::Input, Organelle::Clock],
            AminoAcid::Keep | AminoAcid::Recall => &[Organelle::Store],
            _ => &[],
        }
    }
}
