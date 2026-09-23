//! Canonical-form properties and the S4 four-spellings fixture.

use joinn_dna::{parse_cell, parse_coding, print_coding, sum_cell};
use joinn_frame::FrameRegistry;

fn frames() -> FrameRegistry {
    FrameRegistry::phase1()
}

fn parse_ok(src: &str) -> joinn_dna::CodingRegion {
    match parse_coding(src, &frames()) {
        joinn_frame::Verdict::Ok(c) => c,
        joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn four_sum_spellings_are_byte_identical() {
    let a = r#"
coding {
  frame ℤ 1
  codex 1
  contract {
    join refuse
    port 0 in ℤ 1 required
    port 1 in ℤ 1 required
    port 2 out ℤ 1 required
  }
  # identity first in this spelling
  laws {
    identity: forall a:ℤ 1. self@2(0: a, 1: ℤ 1.zero) = a
    commutative: forall a:ℤ 1 b:ℤ 1. self@2(0: a, 1: b) = self@2(0: b, 1: a)
    associative: forall a:ℤ 1 b:ℤ 1 c:ℤ 1. self@2(0: self@2(0: a, 1: b), 1: c) = self@2(0: a, 1: self@2(0: b, 1: c))
  }
  founding {
    witness { in 0: ℤ 1 2 in 1: ℤ 1 3 out 2: ℤ 1 5 }
  }
  lineage none
}
"#;
    let b = r#"
coding {
codex 1
frame Z 1
contract { join refuse
port 2 out ℤ 1 required
port 1 in ℤ 1 required
port 0 in ℤ 1 required
retired
}
laws {
associative: forall c:ℤ 1 a:ℤ 1 b:ℤ 1. self@2(0: self@2(0: a, 1: b), 1: c) = self@2(0: a, 1: self@2(0: b, 1: c))
identity: forall a:ℤ 1. self@2(0: a, 1: ℤ 1.zero) = a
commutative: forall b:ℤ 1 a:ℤ 1. self@2(0: a, 1: b) = self@2(0: b, 1: a)
}
founding { witness { in 1: ℤ 1 +3 in 0: ℤ 1 2 out 2: ℤ 1 5 } }
lineage none
}
"#;
    let c = print_coding(&parse_ok(a));
    let d = print_coding(&parse_ok(b));
    assert_eq!(c, d);
    let e = print_coding(&sum_cell().coding);
    assert_eq!(c, e);
}

#[test]
fn parse_print_id_and_print_parse_print() {
    let printed = print_coding(&sum_cell().coding);
    let once = parse_ok(&printed);
    assert_eq!(print_coding(&once), printed);
    let twice = parse_ok(&print_coding(&once));
    assert_eq!(print_coding(&twice), printed);
}

#[test]
fn regulatory_perturbation_does_not_change_print() {
    let mut cell = sum_cell();
    let before = print_coding(&cell.coding);
    cell.regulatory.names.insert(0, "left".into());
    cell.regulatory.literals.insert("prompt".into(), "x".into());
    assert_eq!(print_coding(&cell.coding), before);
}

#[test]
fn v24_rename_constant_add_changes() {
    let cell = sum_cell();
    let h = joinn_dna::hash(&cell.coding);
    let mut renamed = cell.clone();
    renamed.regulatory.names.insert(1, "right".into());
    assert_eq!(joinn_dna::hash(&renamed.coding), h);
    let mut added = cell;
    added.coding.contract.ports.push(joinn_dna::PortDecl {
        position: 9,
        direction: joinn_dna::Direction::In,
        frame: joinn_frame::FrameRef::int(),
        required: false,
    });
    assert_ne!(joinn_dna::hash(&added.coding), h);
}

#[test]
fn v19_alleles_do_not_move_hash() {
    let mut cell = sum_cell();
    let h = joinn_dna::hash(&cell.coding);
    cell.alleles.clear();
    assert_eq!(joinn_dna::hash(&cell.coding), h);
    cell.alleles.push(joinn_dna::Allele {
        frame: joinn_frame::FrameRef::rat(),
        body: joinn_dna::AlleleBody::Native(joinn_dna::NativeId("add@ℚ".into())),
        witnesses: Vec::new(),
    });
    assert_eq!(joinn_dna::hash(&cell.coding), h);
}

#[test]
fn cell_file_round_trip() {
    let src = r#"
coding {
  codex 1
  frame ℤ 1
  contract { join refuse port 0 in ℤ 1 required port 1 out ℤ 1 required }
  laws { id: forall a:ℤ 1. self@1(0: a) = a }
  founding { }
  lineage none
}

---

regulatory { names { 0 x  1 y } labels { 0 "the input"  1 "the output" } literals { prompt "n: " } }
"#;
    match parse_cell(src, &frames()) {
        joinn_frame::Verdict::Ok(c) => {
            assert_eq!(c.regulatory.names.get(&0).map(String::as_str), Some("x"));
            assert_eq!(
                c.regulatory.labels.get(&0).map(String::as_str),
                Some("the input")
            );
            assert_eq!(
                c.regulatory.literals.get("prompt").map(String::as_str),
                Some("n: ")
            );
        }
        joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

#[test]
fn optional_turn_does_not_move_codex1_print() {
    let without = print_coding(&sum_cell().coding);
    assert!(
        !without.contains("\nturn\n"),
        "sum_cell must omit the turn block"
    );
    let src = format!("{without}turn\n0 from {{1 2}}\n");
    let parsed = parse_ok(&src);
    let printed = print_coding(&parsed);
    assert!(printed.contains("turn\n0 from {1 2}\n"));
    assert_ne!(printed, without);
    assert_eq!(print_coding(&parse_ok(&without)), without);
}

#[test]
fn sum_cell_hash_is_still_the_golden() {
    let h = joinn_dna::hash(&sum_cell().coding);
    assert_eq!(
        h.to_hex(),
        "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39"
    );
}

#[test]
fn sum_turn_is_an_evolution_of_sum() {
    let src = include_str!("../../../corpus/phase2/sum_turn.cell");
    let cell = match parse_cell(src, &frames()) {
        joinn_frame::Verdict::Ok(c) => c,
        joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let parent = joinn_dna::hash(&sum_cell().coding);
    assert_eq!(cell.coding.lineage, Some(parent));
    assert!(!cell.coding.turns.is_empty());
    assert_eq!(
        joinn_dna::hash(&cell.coding).to_hex(),
        "6fcb1e7397454543fad3b22d4b1c310932985e44d1ed64ae94142ab82339e9cf"
    );
}
