//! What `cargo xtask pool` prints.

use joinn_frame::Verdict;
use joinn_strand::print_letters;
use joinn_tower::{Pool, parse_chromosomes, short_hash};

use super::super::{corpus_artifact, workspace_root};
use super::load_pool;
use super::load_pool::POOL_MEMBERS;

/// The pool, each expansion of twelve's genes with the read count, the two
/// expansion refusals, and the cycle witness refused on entry. An error when
/// a member is refused, something is read before it is asked for, or the
/// cycle enters.
pub(crate) fn pool_text() -> Result<String, String> {
    let (pool, held) = load_pool(&POOL_MEMBERS)?;
    let mut out = String::new();
    for (name, c) in &held {
        let genes: Vec<String> = c
            .genes()
            .iter()
            .map(|(d, h)| format!("node {d} → {}", short_hash(h)))
            .collect();
        out.push_str(&format!(
            "pool: {name} {} order {}, {} gene references{}\n",
            short_hash(&c.identity()),
            c.order(),
            genes.len(),
            if genes.is_empty() {
                String::new()
            } else {
                format!(" ({})", genes.join(", "))
            }
        ));
    }
    if pool.reads() != 0 {
        return Err(format!("pool: {} reads before any expansion", pool.reads()));
    }
    out.push_str(&format!(
        "pool holds {} chromosomes; reads {} (nothing is read until asked)\n",
        pool.len(),
        pool.reads()
    ));
    let Some((_, twelve)) = held.iter().find(|(n, _)| n.ends_with("twelve.chromosome")) else {
        return Err("pool: twelve.chromosome is not held".into());
    };
    for (node, h) in twelve.genes() {
        match pool.expand(twelve, *node) {
            Verdict::Ok(c) => out.push_str(&format!(
                "expand twelve node {node} → {}: {} (reads {})\n",
                short_hash(h),
                print_letters(c.strand()),
                pool.reads()
            )),
            Verdict::Refused(r) => return Err(r.reason),
        }
    }
    match pool.expand(twelve, 3) {
        Verdict::Refused(r) => out.push_str(&format!("expand twelve node 3 → {}\n", r.reason)),
        Verdict::Ok(_) => return Err("pool: node 3 expanded with no gene".into()),
    }
    let (partial, _) = load_pool(&[POOL_MEMBERS[0], POOL_MEMBERS[2]])?;
    match partial.expand(twelve, 5) {
        Verdict::Refused(r) => out.push_str(&format!(
            "expand twelve node 5 in a pool without pair-split → {}\n",
            r.reason
        )),
        Verdict::Ok(_) => return Err("pool: a gene the pool does not hold expanded".into()),
    }
    let name = "phase8/pool/cycle.chromosome";
    let text = std::fs::read_to_string(corpus_artifact(&workspace_root()?, name))
        .map_err(|e| format!("{name}: {e}"))?;
    let both = match parse_chromosomes(&text) {
        Verdict::Ok(all) => all,
        Verdict::Refused(r) => return Err(format!("{name}: {}", r.reason)),
    };
    out.push_str(&format!("cycle: {name}\n"));
    let mut cycle_pool = Pool::default();
    let mut refused = 0;
    for c in both {
        match cycle_pool.insert(c.clone()) {
            Verdict::Ok(h) => out.push_str(&format!("  enters {}\n", short_hash(&h))),
            Verdict::Refused(r) => {
                refused += 1;
                out.push_str(&format!("  refused: {}\n", r.reason));
            }
        }
    }
    if refused != 1 {
        return Err(format!(
            "{out}pool: the cycle witness had {refused} refusals; acceptance is its second chromosome refused"
        ));
    }
    out.push_str("pool: no cycles; grows only when asked\n");
    Ok(out)
}
