//! The pool witness of plan 8 §2.9.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_strand::{StrandFile, parse_letters};
use joinn_tower::{Chromosome, print_chromosome};

/// `pool/cycle.chromosome`: two chromosomes naming each other. A is
/// `AAT AAA TTT ATT` with node 1 naming B; B is `AAT CAC GTG ATT` with node 2
/// naming A. Identity is the strand's hash, so each can name the other.
pub(crate) fn pool_witnesses() -> Result<Vec<(String, String)>, String> {
    let place = |letters: &str, genes: BTreeMap<u64, joinn_frame::Hash>| {
        let strand = match parse_letters(letters) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => return Err(r.reason),
        };
        match Chromosome::new(StrandFile { codex: 1, strand }, genes) {
            Verdict::Ok(c) => Ok(c),
            Verdict::Refused(r) => Err(r.reason),
        }
    };
    let a_plain = place("AAT AAA TTT ATT", BTreeMap::new())?;
    let b = place("AAT CAC GTG ATT", BTreeMap::from([(2, a_plain.identity())]))?;
    let a = place("AAT AAA TTT ATT", BTreeMap::from([(1, b.identity())]))?;
    Ok(vec![(
        "phase8/pool/cycle.chromosome".to_owned(),
        format!("{}\n{}", print_chromosome(&a), print_chromosome(&b)),
    )])
}
