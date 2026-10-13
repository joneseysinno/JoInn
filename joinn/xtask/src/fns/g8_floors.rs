//! Gate 8 item 2: every order holds the one before it.

use joinn_frame::Verdict;
use joinn_tower::{Dim, FACT_IDS, facts, nodes};

use super::dna::tower_witnesses;
use super::{corpus_artifact, workspace_root};

/// The orders checked: T1-T7 for every node and segment, never sampled.
const ORDERS: u32 = 6;

/// T1-T7 hold at orders 1 . 6, so each order's level filter gives the order
/// below; and every tower witness (orders 1-4, `twelve.chromosome`, its
/// floors) is what the fold derives today, byte for byte.
pub(crate) fn g8_floors() -> bool {
    let mut failures = Vec::new();
    for n in 1..=ORDERS {
        let (this, below) = match (nodes(Dim(2), n), nodes(Dim(2), n - 1)) {
            (Verdict::Ok(a), Verdict::Ok(b)) => (a, b),
            (Verdict::Refused(r), _) | (_, Verdict::Refused(r)) => {
                println!("floors: order {n}: {}", r.reason);
                return false;
            }
        };
        for (id, v) in facts(n, &this, &below) {
            if let Verdict::Refused(r) = v {
                failures.push(format!("floors: order {n} {id}: {}", r.reason));
            }
        }
    }
    let root = match workspace_root() {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let witnesses = match tower_witnesses() {
        Ok(w) => w,
        Err(e) => {
            println!("floors: {e}");
            return false;
        }
    };
    for (name, text) in &witnesses {
        match std::fs::read_to_string(corpus_artifact(&root, name)) {
            Ok(on_disk) if on_disk.replace("\r\n", "\n") == *text => {}
            Ok(_) => failures.push(format!(
                "floors: {name} differs from the fold's output; acceptance is the witness byte for byte"
            )),
            Err(e) => failures.push(format!("floors: {name}: {e}")),
        }
    }
    for f in &failures {
        println!("{f}");
    }
    if failures.is_empty() {
        println!(
            "floors: {} facts hold at orders 1 . {ORDERS}; {} tower witnesses derive byte for byte",
            FACT_IDS.len(),
            witnesses.len()
        );
    }
    failures.is_empty()
}
