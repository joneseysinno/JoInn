//! The gene pool of the witnesses, read from the corpus.

use joinn_frame::Verdict;
use joinn_tower::{Chromosome, Pool, parse_chromosomes};

use super::super::{corpus_artifact, workspace_root};

/// The pool's members: `twelve.chromosome` and the two strands it names.
pub(crate) const POOL_MEMBERS: [&str; 3] = [
    "phase8/strands/empty.strand",
    "phase8/strands/pair-split.strand",
    "phase8/tower/twelve.chromosome",
];

/// Each member `members` names, entered in order; a refusal is the error.
pub(crate) fn load_pool(members: &[&str]) -> Result<(Pool, Vec<(String, Chromosome)>), String> {
    let root = workspace_root()?;
    let mut pool = Pool::default();
    let mut held = Vec::new();
    for name in members {
        let path = corpus_artifact(&root, name);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{name}: {e}"))?;
        let all = match parse_chromosomes(&text) {
            Verdict::Ok(all) => all,
            Verdict::Refused(r) => return Err(format!("{name}: {}", r.reason)),
        };
        for c in all {
            if let Verdict::Refused(r) = pool.insert(c.clone()) {
                return Err(format!("{name}: {}", r.reason));
            }
            held.push(((*name).to_owned(), c));
        }
    }
    Ok((pool, held))
}
