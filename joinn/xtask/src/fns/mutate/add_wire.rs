//! Add an internal body wire by endpoints. Frames are the body check's business.

use joinn_dna::{Body, Wire};
use joinn_frame::Verdict;

use super::parse_endpoint::parse_endpoint;
use super::refuse::refuse;

pub(super) fn add_wire(body: &mut Body, src_s: &str, dst_s: &str) -> Verdict<()> {
    let (src_inst, src_port) = match parse_endpoint(src_s) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let (dst_inst, dst_port) = match parse_endpoint(dst_s) {
        Verdict::Ok(p) => p,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    for inst in [&src_inst, &dst_inst] {
        if !body
            .coding
            .genome
            .iter()
            .any(|e| e.instances.iter().any(|i| i == inst))
        {
            return Verdict::Refused(refuse(format!(
                "no such instance {inst}; acceptance is a genome instance"
            )));
        }
    }
    let wire = Wire {
        src_instance: src_inst,
        src_port,
        dst_instance: dst_inst,
        dst_port,
    };
    if body.coding.wires.contains(&wire) {
        return Verdict::Refused(refuse(format!(
            "wire {src_s} -> {dst_s} is already declared; acceptance is a wire the body lacks"
        )));
    }
    body.coding.wires.push(wire);
    Verdict::Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fns::mutate::corpus_fixture::corpus_fixture;
    use crate::fns::mutate::{Mutation, mutate};
    use crate::fns::parse_subject::parse_subject;
    use crate::fns::subject::Subject;
    use crate::fns::workspace_root::workspace_root;
    use joinn_dna::hash;
    use joinn_frame::Verdict;
    use joinn_link::{
        BodyBinding, BodyStore, Universe, UniverseCoding, UniverseRegulatory, assay, bind,
        print_assay,
    };
    use std::collections::BTreeMap;
    use std::fs;

    const JOINED: &str = "\
assay body
regions: body 1
islands: 1
loops: 3
filled: outside →body.answer@1→ body →body.answer@2→ outside by frame ℤ
filled: outside →body.question@0→ body →body.answer@2→ outside by frame ℤ
filled: outside →body.question@1→ body →body.answer@2→ outside by frame ℤ
not measured: 0
H₀: 1
H₁: 0
H₂: 0
euler: V 2 − E 4 + F 3 = 1 = 1 − 0 + 0
";

    fn read(rel: &str) -> Subject {
        let root = workspace_root().unwrap_or_else(|e| panic!("{e}"));
        let text =
            fs::read_to_string(root.join("corpus").join(rel)).unwrap_or_else(|e| panic!("{e}"));
        parse_subject(rel, &text).unwrap_or_else(|e| panic!("{e}"))
    }

    fn report(subject: &Subject) -> String {
        let Subject::Body(body) = subject else {
            panic!("body");
        };
        let (cells, _) = corpus_fixture(&[]);
        let mut store = BodyStore::new();
        match store.insert(body.clone(), cells, "add wire") {
            Verdict::Ok(_) => {}
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
        let universe = Universe {
            coding: UniverseCoding {
                codex: 1,
                declarations: Vec::new(),
                bodies: vec![BodyBinding {
                    hash: hash(&body.coding),
                    alias: "body".to_string(),
                }],
                links: Vec::new(),
                cross_wires: Vec::new(),
                grants: BTreeMap::new(),
                lenses: Vec::new(),
            },
            regulatory: UniverseRegulatory::default(),
        };
        let bound = match bind(&universe, &store) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        match assay(&universe, &bound) {
            Verdict::Ok(r) => print_assay(&r),
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn add_wire_joins_the_asker_into_one_region() {
        let asker = read("phase52/adversary/asker.body");
        let before = report(&asker);
        assert!(
            before.contains("regions: body 2 {answer} {question}\n"),
            "{before}"
        );
        let joined = match mutate(&asker, &Mutation::AddWire("question@2", "answer@0")) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let after = report(&joined);
        eprintln!("{after}");
        assert_eq!(after, JOINED);

        let Verdict::Refused(r) = mutate(&asker, &Mutation::AddWire("ghost@2", "answer@0")) else {
            panic!("a missing instance must refuse");
        };
        assert!(r.reason.contains("ghost"), "{}", r.reason);
        let Verdict::Refused(r) = mutate(&joined, &Mutation::AddWire("question@2", "answer@0"))
        else {
            panic!("an existing wire must refuse");
        };
        assert!(r.reason.contains("question@2 -> answer@0"), "{}", r.reason);
    }
}
