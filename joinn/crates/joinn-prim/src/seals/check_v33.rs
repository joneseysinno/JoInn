//! Walk a reference body. Mutant 14.

use joinn_dna::{Body, NativeId};
use joinn_frame::Verdict;
use std::collections::BTreeSet;

/// Walk a reference body. Mutant 14.
pub fn check_v33(body: &Body, sealed: &NativeId, floor: &BTreeSet<String>) -> Verdict<()> {
    joinn_gate::check::v33::check_simple(body, &sealed.0, floor)
}

#[cfg(test)]
mod tests {
    use super::check_v33;
    use joinn_dna::NativeId;
    use joinn_frame::{CheckId, Verdict};
    use std::collections::BTreeSet;

    #[test]
    fn v33_refuses_a_genome_that_names_the_sealed_native() {
        let mut floor = BTreeSet::new();
        floor.insert("eq".into());
        let src = "body { codex 1 genome { prim:eq as e } grants { } wires { } budget { steps 1 } lineage none }\n";
        let body = match joinn_dna::parse_body(src, &joinn_frame::FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match check_v33(&body, &NativeId("eq".into()), &floor) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains("check::v33"), "{}", r.reason);
                assert_eq!(r.check, CheckId::V33);
            }
            Verdict::Ok(()) => panic!("mutant 14 must be refused by check::v33"),
        }
    }

    #[test]
    fn v33_accepts_a_prim_only_body() {
        let mut floor = BTreeSet::new();
        floor.insert("eq".into());
        floor.insert("case".into());
        let src = "body { codex 1 genome { prim:eq as e prim:case as c } grants { } wires { } budget { steps 1 } lineage none }\n";
        let body = match joinn_dna::parse_body(src, &joinn_frame::FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match check_v33(&body, &NativeId("add@ℤ".into()), &floor) {
            Verdict::Ok(()) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
}
