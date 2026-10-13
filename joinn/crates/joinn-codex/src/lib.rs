//! Codons, codex 1, the organelle list and the reader (plan 8 §2.3–§2.5,
//! §2.8). The table lives here and nowhere else (D11).

#![forbid(unsafe_code)]

mod codex;
mod codon;
mod meaning;
mod organelle;
mod reader;
mod refuse;
mod side;

pub use codex::{CODEX, CODEX_1, READER_VERSION, Row, check_table, codon_of, lookup};
pub use codon::{Codon, Parity};
pub use meaning::{AminoAcid, Meaning};
pub use organelle::{ORGANELLES, Organelle};
pub use reader::{Protein, meanings, print_proteins, read_file, translate};
pub use side::{Side, side};
