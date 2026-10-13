//! An amino acid's opposite.

use super::AminoAcid;

impl AminoAcid {
    /// The acid on the other side of its table row (D19).
    pub fn opposite(self) -> AminoAcid {
        match self {
            AminoAcid::Pair => AminoAcid::Split,
            AminoAcid::Split => AminoAcid::Pair,
            AminoAcid::Build => AminoAcid::Case,
            AminoAcid::Case => AminoAcid::Build,
            AminoAcid::Refer => AminoAcid::Return,
            AminoAcid::Return => AminoAcid::Refer,
            AminoAcid::Sense => AminoAcid::Show,
            AminoAcid::Show => AminoAcid::Sense,
            AminoAcid::Keep => AminoAcid::Recall,
            AminoAcid::Recall => AminoAcid::Keep,
            AminoAcid::Grant => AminoAcid::Revoke,
            AminoAcid::Revoke => AminoAcid::Grant,
        }
    }
}
