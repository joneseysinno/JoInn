//! Coding region, regulatory region, laws, witnesses, canonical text, hashing.
//!
//! This crate is data. It never evaluates a law and never holds a function pointer.

#![forbid(unsafe_code)]

pub mod body;
pub mod formula;
pub mod hash;
pub mod model;
pub mod parse;
pub mod print;

pub use body::{
    Body, BodyCoding, BodyRegulatory, GenomeEntry, GenomeTarget, Wire, check_body, parse_body,
    print_body,
};
pub use formula::{Formula, Law, LawName, Term_, VarId};
pub use hash::{Genotype, hash};
pub use model::{
    Allele, AlleleBody, Cell, CodingRegion, Contract, Declaration, Direction, JoinPolicy, NativeId,
    PortDecl, RegulatoryRegion, TurnDecl, Witness, cli_input_cell, format_cell, sum_cell,
};
pub use parse::{parse_cell, parse_coding};
pub use print::{print_allele, print_coding, print_formula, print_term_};

#[cfg(test)]
mod tests {
    use super::*;
    use joinn_frame::{FrameRegistry, TAG_CELL, keyed_hash};

    #[test]
    fn regulatory_cannot_be_hashed() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/hash_regulatory.rs");
        t.compile_fail("tests/fail/hash_body_regulatory.rs");
    }

    #[test]
    fn hash_is_blake3_of_canonical_text() {
        let frames = FrameRegistry::phase1();
        let cell = crate::model::sum_cell();
        let text = print_coding(&cell.coding);
        let h = hash(&cell.coding);
        assert_eq!(h, keyed_hash(TAG_CELL, text.as_bytes()));
        let _ = frames;
    }

    #[test]
    fn parse_print_round_trip_sum() {
        let frames = FrameRegistry::phase1();
        let cell = crate::model::sum_cell();
        let printed = print_coding(&cell.coding);
        let parsed = match parse_coding(&printed, &frames) {
            joinn_frame::Verdict::Ok(c) => c,
            joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(print_coding(&parsed), printed);
        let again = match parse_coding(&print_coding(&parsed), &frames) {
            joinn_frame::Verdict::Ok(c) => c,
            joinn_frame::Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(print_coding(&again), printed);
    }

    #[test]
    fn primitive_name_is_a_parse_refusal() {
        let frames = FrameRegistry::phase1();
        let src = "codex 1\ncontract\nensure\njoin refuse\nport 0 in ℤ 1 required\nport 1 out ℤ 1 required\nrequire\nretired\ndeclarations\nfounding\nframe ℤ 1\nlaws\nbad: forall a:ℤ 1. int.add(a) = a\nlineage none\n";
        match parse_coding(src, &frames) {
            joinn_frame::Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("primitive") || r.reason.contains("int"),
                    "{}",
                    r.reason
                );
            }
            joinn_frame::Verdict::Ok(_) => panic!("int.add must be a parse refusal"),
        }
    }
}
