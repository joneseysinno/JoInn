//! `--strand <file>`: the strand view of a `.strand` file.

use std::path::Path;

use super::run_view::run_view;
use crate::load::load_strand_file;
use crate::session::Dna;

/// The strand beside the reader's partner, then one tick per change.
pub fn run_dna(path: &str) -> Result<(), String> {
    let file = load_strand_file(Path::new(path))?;
    run_view(Box::new(Dna::open(&file.strand)?))
}
