//! Drop an internal body wire by endpoints.

use joinn_dna::Body;
use joinn_frame::Verdict;

use super::parse_endpoint::parse_endpoint;
use super::refuse::refuse;

pub(super) fn drop_wire(body: &mut Body, src_s: &str, dst_s: &str) -> Verdict<()> {
    let (src_inst, src_port) = match parse_endpoint(src_s) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let (dst_inst, dst_port) = match parse_endpoint(dst_s) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let before = body.coding.wires.len();
    body.coding.wires.retain(|w| {
        !(w.src_instance == src_inst
            && w.src_port == src_port
            && w.dst_instance == dst_inst
            && w.dst_port == dst_port)
    });
    if body.coding.wires.len() == before {
        return Verdict::Refused(refuse(format!(
            "no such wire {src_s} -> {dst_s}; acceptance is a declared wire"
        )));
    }
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::load_calculator::load_calculator;
    use crate::fns::mutate::{mutate, Mutation};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use joinn_dna::hash;
    use joinn_frame::Verdict;
    use joinn_link::membrane;
    use std::collections::BTreeSet;

    fn calculator() -> Subject {
        let src = include_str!("../../../../corpus/phase2/calculator.body");
        parse_subject("phase2/calculator.body", src).expect("parse")
    }

    #[test]
    fn drop_wire_changes_membrane() {
        let s = calculator();
        let Subject::Body(orig) = &s else {
            panic!("body");
        };
        let orig_hash = hash(&orig.coding);
        let Verdict::Ok(mutant) = mutate(&s, &Mutation::DropWire("cli_a@1", "sum@0")) else {
            panic!("mutate");
        };
        let Subject::Body(body) = &mutant else {
            panic!("body mutant");
        };
        assert_ne!(hash(&body.coding), orig_hash);
        let (_, cells) = load_calculator().expect("cells");
        let set = match membrane(body, &cells) {
            Verdict::Ok(set) => set,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let got: BTreeSet<String> = set.iter().map(|a| a.address.printed()).collect();
        let expected = BTreeSet::from([
            "cli_a@0".to_string(),
            "cli_b@0".to_string(),
            "sum@2".to_string(),
        ]);
        assert_ne!(got, expected, "membrane must change after DropWire");
        let Verdict::Refused(r) = mutate(&s, &Mutation::DropWire("ghost@0", "sum@0")) else {
            panic!("missing wire must refuse");
        };
        assert!(r.reason.contains("ghost@0"), "{}", r.reason);
    }
}
