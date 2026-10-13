//! twelve.chromosome and the witness pool, as the shell loads them.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_strand::parse_strand_file;
use joinn_tower::{Chromosome, Pool, parse_chromosomes};

use super::Dna;

/// twelve.chromosome opened as its strand (`folded` false) or its fold, over
/// a pool of twelve, pair-split and empty.
pub(super) fn twelve(folded: bool) -> Dna {
    let twelve = match parse_chromosomes(include_str!(
        "../../../../corpus/phase8/tower/twelve.chromosome"
    )) {
        Verdict::Ok(mut c) if c.len() == 1 => c.remove(0),
        Verdict::Ok(_) => panic!("one chromosome"),
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut pool = Pool::default();
    for src in [
        include_str!("../../../../corpus/phase8/strands/pair-split.strand"),
        include_str!("../../../../corpus/phase8/strands/empty.strand"),
    ] {
        let gene = match parse_strand_file(src) {
            Verdict::Ok(f) => match Chromosome::new(f, BTreeMap::new()) {
                Verdict::Ok(c) => c,
                Verdict::Refused(r) => panic!("{}", r.reason),
            },
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        if let Verdict::Refused(r) = pool.insert(gene) {
            panic!("{}", r.reason);
        }
    }
    if let Verdict::Refused(r) = pool.insert(twelve.clone()) {
        panic!("{}", r.reason);
    }
    match Dna::open(twelve, pool, folded) {
        Ok(d) => d,
        Err(e) => panic!("{e}"),
    }
}
