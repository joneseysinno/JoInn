//! `--strand <file>` and `--fold <file>`: the two pictures of a chromosome.

use std::path::Path;

use super::run_view::run_view;
use crate::load::{find_corpus, load_dna};
use crate::session::Dna;

/// The strand view (`folded` false) or the fold view of a `.strand` or
/// `.chromosome` file, Tab between them, then one tick per change.
pub fn run_dna(path: &str, folded: bool) -> Result<(), String> {
    let corpus = find_corpus()?;
    let (chromosome, pool) = load_dna(Path::new(path), &corpus)?;
    run_view(Box::new(Dna::open(chromosome, pool, folded)?))
}
