//! An amino acid's printed name.

use super::AminoAcid;

impl AminoAcid {
    /// Lower case, as in the table.
    pub fn name(self) -> &'static str {
        match self {
            AminoAcid::Pair => "pair",
            AminoAcid::Split => "split",
            AminoAcid::Build => "build",
            AminoAcid::Case => "case",
            AminoAcid::Refer => "refer",
            AminoAcid::Return => "return",
            AminoAcid::Sense => "sense",
            AminoAcid::Show => "show",
            AminoAcid::Keep => "keep",
            AminoAcid::Recall => "recall",
            AminoAcid::Grant => "grant",
            AminoAcid::Revoke => "revoke",
        }
    }
}
