//! Print a strand file.

use super::{StrandFile, print_letters};

/// `codex <n>`, then the letters in threes, each line ending in `\n`.
pub fn print_strand_file(file: &StrandFile) -> String {
    format!("codex {}\n{}\n", file.codex, print_letters(&file.strand))
}
