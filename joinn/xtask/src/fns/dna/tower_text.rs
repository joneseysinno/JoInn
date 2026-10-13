//! What `cargo xtask tower` prints.

use joinn_frame::Verdict;
use joinn_tower::{Dim, facts, nodes};

use super::{floors_text, witness_chromosomes};

/// T1–T7 for orders 1–6, each fact's line or its refusal; every witness
/// chromosome by floor; the dimension refusal. The whole text is the error
/// when any fact fails.
pub(crate) fn tower_text() -> Result<String, String> {
    let mut out = String::from("tower: dimension 2, orders 1-6\n");
    let mut failed = 0;
    for order in 1..=6u32 {
        let (this, below) = match (nodes(Dim(2), order), nodes(Dim(2), order - 1)) {
            (Verdict::Ok(a), Verdict::Ok(b)) => (a, b),
            _ => return Err(format!("tower: order {order} has no nodes")),
        };
        out.push_str(&format!(
            "order {order}: {} nodes, {} segments\n",
            this.len(),
            this.len() - 1
        ));
        for (id, v) in facts(order, &this, &below) {
            match v {
                Verdict::Ok(line) => out.push_str(&format!("  {id} {line}\n")),
                Verdict::Refused(r) => {
                    failed += 1;
                    out.push_str(&format!("  {id} refused: {}\n", r.reason));
                }
            }
        }
    }
    for (name, chromosomes) in witness_chromosomes()? {
        for c in chromosomes {
            out.push_str(&format!("{name}\n"));
            for line in floors_text(&c).lines() {
                out.push_str(&format!("  {line}\n"));
            }
        }
    }
    match nodes(Dim(4), 1) {
        Verdict::Refused(r) => out.push_str(&format!("dimension 4: {}\n", r.reason)),
        Verdict::Ok(_) => {
            failed += 1;
            out.push_str("dimension 4: built; acceptance is a refusal\n");
        }
    }
    if failed > 0 {
        out.push_str(&format!(
            "tower: {failed} fact(s) fail; acceptance is T1-T7 at orders 1-6\n"
        ));
        return Err(out);
    }
    out.push_str("tower: orders 1-6 hold T1-T7; every order holds the one before it\n");
    Ok(out)
}
