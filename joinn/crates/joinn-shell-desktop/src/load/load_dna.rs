//! Read the `.strand` or `.chromosome` file the shell was given, and the pool.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use joinn_frame::Verdict;
use joinn_strand::parse_strand_file;
use joinn_tower::{Chromosome, Pool, parse_chromosomes};

/// The chromosome to draw and the pool its genes open from. The pool holds
/// every chromosome in `corpus/phase8/tower/` and then every strand in
/// `corpus/phase8/strands/` whose hash it does not already hold, in path
/// order; a refusal there is the pool's and stops the shell. A `.chromosome`
/// opens its first block; a `.strand` opens as the tower chromosome with its
/// hash when there is one (so its rings show), else with no gene references.
pub(crate) fn load_dna(path: &Path, corpus: &Path) -> Result<(Chromosome, Pool), String> {
    let read = |p: &Path| fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()));
    let listed = |dir: PathBuf, ext: &str| -> Result<Vec<PathBuf>, String> {
        let mut out = Vec::new();
        for ent in fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))? {
            let p = ent.map_err(|e| e.to_string())?.path();
            if p.extension().is_some_and(|e| e == ext) {
                out.push(p);
            }
        }
        out.sort();
        Ok(out)
    };
    let as_chromosome = |p: &Path, text: &str| -> Result<Chromosome, String> {
        let file = match parse_strand_file(text) {
            Verdict::Ok(f) => f,
            Verdict::Refused(r) => return Err(format!("{}: {}", p.display(), r.reason)),
        };
        match Chromosome::new(file, BTreeMap::new()) {
            Verdict::Ok(c) => Ok(c),
            Verdict::Refused(r) => Err(format!("{}: {}", p.display(), r.reason)),
        }
    };
    let phase8 = corpus.join("phase8");
    let mut towers = Vec::new();
    for p in listed(phase8.join("tower"), "chromosome")? {
        match parse_chromosomes(&read(&p)?) {
            Verdict::Ok(cs) => towers.extend(cs),
            Verdict::Refused(r) => return Err(format!("{}: {}", p.display(), r.reason)),
        }
    }
    let text = read(path)?;
    let shown = if path.extension().is_some_and(|e| e == "chromosome") {
        match parse_chromosomes(&text) {
            Verdict::Ok(cs) => cs.into_iter().next().ok_or_else(|| {
                format!(
                    "{}: holds no chromosome; acceptance is a strand file and its gene references",
                    path.display()
                )
            })?,
            Verdict::Refused(r) => return Err(format!("{}: {}", path.display(), r.reason)),
        }
    } else {
        let bare = as_chromosome(path, &text)?;
        towers
            .iter()
            .find(|c| c.identity() == bare.identity())
            .cloned()
            .unwrap_or(bare)
    };
    let mut pool = Pool::default();
    for c in towers {
        if let Verdict::Refused(r) = pool.insert(c) {
            return Err(r.reason);
        }
    }
    for p in listed(phase8.join("strands"), "strand")? {
        let c = as_chromosome(&p, &read(&p)?)?;
        if pool.hashes().contains(&c.identity()) {
            continue;
        }
        if let Verdict::Refused(r) = pool.insert(c) {
            return Err(format!("{}: {}", p.display(), r.reason));
        }
    }
    Ok((shown, pool))
}

#[cfg(test)]
mod tests {
    use super::load_dna;
    use crate::load::find_corpus;

    #[test]
    fn twelve_strand_opens_with_its_rings_over_a_pool_of_four() {
        let corpus = match find_corpus() {
            Ok(c) => c,
            Err(e) => panic!("{e}"),
        };
        let (shown, pool) = match load_dna(&corpus.join("phase8/strands/twelve.strand"), &corpus) {
            Ok(d) => d,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(shown.genes().keys().copied().collect::<Vec<_>>(), [5, 10]);
        assert_eq!(
            pool.len(),
            4,
            "twelve.chromosome, empty, pair-split, reserved"
        );
        assert_eq!(pool.reads(), 0);
    }
}
