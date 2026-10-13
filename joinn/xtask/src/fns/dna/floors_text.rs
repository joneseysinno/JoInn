//! A chromosome at every floor, as its witness holds it.

use joinn_codex::{Meaning, lookup};
use joinn_tower::Chromosome;

/// A header line, then one line per floor 0 … order − 1: the codons on
/// segments of level ≥ f, each with its segment and meaning.
pub(crate) fn floors_text(c: &Chromosome) -> String {
    let codons = c.stored_codons() as u64;
    let mut out = format!(
        "chromosome {} order {}: {codons} codons on {} segments, {} silent, {} gene references\n",
        c.identity().to_hex(),
        c.order(),
        c.segments(),
        c.segments() - codons,
        c.genes().len()
    );
    for f in 0..c.order() {
        let kept: Vec<String> = c
            .floor(f)
            .into_iter()
            .map(|(s, k)| {
                format!(
                    "{s} {} {}",
                    k.text(),
                    lookup(k).map_or("reserved", Meaning::name)
                )
            })
            .collect();
        out.push_str(&format!(
            "floor {f} (level ≥ {f}): {} codons: {}\n",
            kept.len(),
            kept.join(", ")
        ));
    }
    out
}
