//! The chromosome witnesses, read from the corpus.

use joinn_frame::Verdict;
use joinn_tower::{Chromosome, parse_chromosomes};

use super::super::{corpus_artifact, workspace_root};

/// The corpus names of every chromosome witness.
pub(crate) const CHROMOSOME_WITNESSES: [&str; 1] = ["phase8/tower/twelve.chromosome"];

/// Each witness's name and its chromosomes, as committed.
pub(crate) fn witness_chromosomes() -> Result<Vec<(String, Vec<Chromosome>)>, String> {
    let root = workspace_root()?;
    let mut out = Vec::new();
    for name in CHROMOSOME_WITNESSES {
        let path = corpus_artifact(&root, name);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{name}: {e}"))?;
        match parse_chromosomes(&text) {
            Verdict::Ok(all) => out.push((name.to_owned(), all)),
            Verdict::Refused(r) => return Err(format!("{name}: {}", r.reason)),
        }
    }
    Ok(out)
}
