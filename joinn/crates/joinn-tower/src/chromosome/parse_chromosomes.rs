//! Read a chromosome file.

use std::collections::BTreeMap;

use joinn_frame::{Hash, Verdict};
use joinn_strand::parse_strand_file;

use super::Chromosome;
use crate::refuse::refuse;

/// One or more blocks separated by a blank line. Each block is a strand file
/// (`codex <n>`, the letters) followed by `node <d> gene <hash>` lines sorted
/// by d, one per node.
pub fn parse_chromosomes(text: &str) -> Verdict<Vec<Chromosome>> {
    let text = text.replace("\r\n", "\n");
    let mut out = Vec::new();
    for block in text.split("\n\n").filter(|b| !b.trim().is_empty()) {
        let lines: Vec<&str> = block.lines().collect();
        let head = lines.iter().take(2).copied().collect::<Vec<_>>().join("\n");
        let file = match parse_strand_file(&format!("{head}\n")) {
            Verdict::Ok(f) => f,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let mut genes: BTreeMap<u64, Hash> = BTreeMap::new();
        for line in lines.iter().skip(2) {
            let parts: Vec<&str> = line.split(' ').collect();
            let (node, hash) = match parts.as_slice() {
                ["node", d, "gene", h] => match (d.parse::<u64>(), Hash::parse_hex(h)) {
                    (Ok(d), Some(h)) => (d, h),
                    _ => {
                        return refuse(format!(
                            "tower: {line:?} is not a gene reference; acceptance is `node <d> gene <64 hex>`"
                        ));
                    }
                },
                _ => {
                    return refuse(format!(
                        "tower: {line:?} is not a gene reference; acceptance is `node <d> gene <64 hex>`"
                    ));
                }
            };
            if let Some((last, _)) = genes.last_key_value() {
                if node <= *last {
                    return refuse(format!(
                        "tower: node {node} follows node {last}; acceptance is gene references sorted by node, one per node"
                    ));
                }
            }
            genes.insert(node, hash);
        }
        match Chromosome::new(file, genes) {
            Verdict::Ok(c) => out.push(c),
            Verdict::Refused(r) => return Verdict::Refused(r),
        }
    }
    Verdict::Ok(out)
}
