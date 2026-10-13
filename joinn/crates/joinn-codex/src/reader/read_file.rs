//! Read a strand file.

use joinn_frame::Verdict;
use joinn_strand::StrandFile;

use super::{Protein, translate};
use crate::codex::CODEX;
use crate::refuse::refuse;

/// The file's `codex` line must name the codex this reader holds.
pub fn read_file(file: &StrandFile) -> Verdict<Vec<Protein>> {
    if file.codex != CODEX {
        return refuse(format!(
            "reader: strand names codex {}; this reader holds codex {CODEX}",
            file.codex
        ));
    }
    translate(&file.strand)
}
