//! Simplified V33 check.

use joinn_dna::Body;
use joinn_frame::{Hash, Verdict};
use std::collections::{BTreeMap, BTreeSet};

/// Compatibility wrapper used by mutant 14 and older callers.
pub fn check_simple(body: &Body, sealed: &str, floor: &BTreeSet<String>) -> Verdict<()> {
    let matter = floor.clone();
    super::check(
        body,
        sealed,
        floor,
        &matter,
        Hash::from_bytes([0; 32]),
        &[],
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use joinn_dna::parse_body;
    use joinn_frame::{CheckId, FrameRegistry};
    use std::collections::BTreeSet;

    #[test]
    fn names_the_sealed_native() {
        let src = "body { codex 1 genome { prim:eq as e } grants { } wires { } budget { steps 1 } lineage none }\n";
        let body = match parse_body(src, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut floor = BTreeSet::new();
        floor.insert("eq".into());
        match check_simple(&body, "eq", &floor) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("check::v33"), "{}", r.reason);
                assert_eq!(r.check, CheckId::V33);
            }
            Verdict::Ok(()) => panic!("naming the sealed native must refuse"),
        }
    }
}
