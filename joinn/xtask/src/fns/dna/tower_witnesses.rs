//! The tower witnesses of plan 8 §2.9.

use std::collections::BTreeMap;

use joinn_frame::Verdict;
use joinn_strand::parse_strand_file;
use joinn_tower::{Chromosome, print_chromosome};

use super::{floors_text, order_text, strand_witnesses};

/// Orders 1–4; `twelve.strand` on its tower with two gene references (node 5
/// names `pair-split`, node 10 names `empty`); and its floors.
pub(crate) fn tower_witnesses() -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    for order in 1..=4 {
        out.push((
            format!("phase8/tower/order-{order}.txt"),
            order_text(order)?,
        ));
    }
    let strands = strand_witnesses()?;
    let file = |name: &str| {
        let key = format!("phase8/strands/{name}.strand");
        let text = strands
            .iter()
            .find(|(n, _)| *n == key)
            .map(|(_, t)| t.clone())
            .ok_or(format!("no strand witness {key}"))?;
        match parse_strand_file(&text) {
            Verdict::Ok(f) => Ok(f),
            Verdict::Refused(r) => Err(r.reason),
        }
    };
    let genes = BTreeMap::from([
        (5, file("pair-split")?.strand.identity()),
        (10, file("empty")?.strand.identity()),
    ]);
    let twelve = match Chromosome::new(file("twelve")?, genes) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => return Err(r.reason),
    };
    out.push((
        "phase8/tower/twelve.chromosome".to_owned(),
        print_chromosome(&twelve),
    ));
    out.push((
        "phase8/tower/twelve.floors.txt".to_owned(),
        floors_text(&twelve),
    ));
    Ok(out)
}
