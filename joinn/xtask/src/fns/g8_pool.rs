//! Gate 8 item 3: the pool has no cycles.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_strand::{StrandFile, parse_letters};
use joinn_tower::{Chromosome, Pool};

use super::dna::{pool_text, pool_witnesses};
use super::{corpus_artifact, workspace_root};

/// The cycle witness is what the pool witness writes today, byte for byte;
/// the pool reads nothing until asked and refuses the cycle at its second
/// chromosome (`cargo xtask pool`); and a gene naming itself is refused on
/// entry to an empty pool.
pub(crate) fn g8_pool() -> bool {
    let mut failures = Vec::new();
    let root = match workspace_root() {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    match pool_witnesses() {
        Ok(witnesses) => {
            for (name, text) in &witnesses {
                match std::fs::read_to_string(corpus_artifact(&root, name)) {
                    Ok(on_disk) if on_disk.replace("\r\n", "\n") == *text => {}
                    Ok(_) => failures.push(format!(
                        "pool: {name} differs from the pool witness; acceptance is the witness byte for byte"
                    )),
                    Err(e) => failures.push(format!("pool: {name}: {e}")),
                }
            }
        }
        Err(e) => failures.push(format!("pool: {e}")),
    }
    if let Err(e) = pool_text() {
        failures.push(e);
    }
    let strand = match parse_letters("AAT AAA TTT ATT") {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => {
            println!("pool: {}", r.reason);
            return false;
        }
    };
    let me = strand.identity();
    let itself = match Chromosome::new(StrandFile { codex: 1, strand }, BTreeMap::from([(1, me)])) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => {
            println!("pool: {}", r.reason);
            return false;
        }
    };
    let self_line = match Pool::default().insert(itself) {
        Verdict::Refused(r) => r.reason,
        Verdict::Ok(_) => {
            failures.push(
                "pool: a gene naming itself entered; acceptance is a pool without cycles".into(),
            );
            String::new()
        }
    };
    for f in &failures {
        println!("{f}");
    }
    if failures.is_empty() {
        println!("pool: the cycle witness is refused at its second chromosome; {self_line}");
    }
    failures.is_empty()
}
