//! Body form: genome, grants, wires, budget. Data only.

#![allow(clippy::result_large_err)]

mod check_body;
mod close_section;
mod coding_body;
mod expect;
mod hex_hash;
mod ident;
mod number_u32;
mod number_u64;
mod parse_body;
mod parse_body_coding;
mod parse_body_regulatory;
mod peek_ident;
mod print_body;
mod quoted;
mod section_end;
mod split_delimiter;
mod strip_comments;
mod take_brace;

pub use check_body::check_body;
pub use parse_body::parse_body;
pub use print_body::print_body;

use joinn_frame::{CheckId, Hash, Refusal, Subject};
use std::collections::{BTreeMap, BTreeSet};

/// One genome entry: a cell hash or a floor member, bound to instance names.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GenomeTarget {
    /// Full 64-hex cell identity.
    Cell(Hash),
    /// A floor member, named. Resolves against the floor register.
    Prim(String),
}

/// One genome entry: a target bound to instance names.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct GenomeEntry {
    /// Cell hash or floor member.
    pub target: GenomeTarget,
    /// Instance names, coding, ordered.
    pub instances: Vec<String>,
}

/// A wire from an out-port to an in-port. Self-wires are permitted.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Wire {
    /// Source instance.
    pub src_instance: String,
    /// Source port position.
    pub src_port: u32,
    /// Destination instance.
    pub dst_instance: String,
    /// Destination port position.
    pub dst_port: u32,
}

/// Hashed half of a body.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BodyCoding {
    /// Canonical-form version. Phase 2 is 1.
    pub codex: u16,
    /// Cells in this body.
    pub genome: Vec<GenomeEntry>,
    /// Capability name → instance names, in grant order.
    pub grants: BTreeMap<String, Vec<String>>,
    /// Environment signals this body reads. Omitted from print when empty.
    pub reads: BTreeSet<String>,
    /// Wires.
    pub wires: Vec<Wire>,
    /// Step budget.
    pub budget_steps: u64,
    /// Parent body hash, if any.
    pub lineage: Option<Hash>,
}

/// Prompts, present templates, instance display names. Never hashed.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BodyRegulatory {
    /// Instance → prompt string.
    pub prompts: BTreeMap<String, String>,
    /// Instance → present template.
    pub present: BTreeMap<String, String>,
    /// Instance display names. Regulatory.
    pub names: BTreeMap<String, String>,
    /// Instance accessibility labels. Regulatory.
    pub labels: BTreeMap<String, String>,
}

/// A body: coding + regulatory.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Body {
    /// Hashed.
    pub coding: BodyCoding,
    /// Never hashed.
    pub regulatory: BodyRegulatory,
}

/// Cursor over NFC-normalized body source.
pub(in crate::body) struct BodyParser<'a> {
    src: &'a str,
    i: usize,
}

impl<'a> BodyParser<'a> {
    pub(in crate::body) fn new(src: &'a str) -> Self {
        Self { src, i: 0 }
    }

    pub(in crate::body) fn eof(&self) -> bool {
        self.i >= self.src.len()
    }

    pub(in crate::body) fn rest(&self) -> &'a str {
        &self.src[self.i..]
    }

    pub(in crate::body) fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    pub(in crate::body) fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.i += c.len_utf8();
        }
    }

    pub(in crate::body) fn skip(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    pub(in crate::body) fn refuse(&self, reason: impl Into<String>) -> Refusal {
        Refusal {
            check: CheckId::Parse,
            subject: Subject::Other(String::new()),
            reason: reason.into(),
            counterexample: None,
            seed: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use joinn_frame::{FrameRegistry, TAG_BODY, Verdict, keyed_hash};

    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";

    fn frames() -> FrameRegistry {
        FrameRegistry::phase1()
    }

    fn parse_ok(src: &str) -> Body {
        match parse_body(src, &frames()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn four_spellings_canonicalize_byte_identically() {
        let a = format!(
            r#"
body {{
  codex 1
  genome {{
    cell:{SUM}  as sum
    cell:{CLI}  as cli_a, cli_b
  }}
  grants {{
    stdin: cli_a, cli_b
  }}
  wires {{
    cli_a@1 -> sum@0
    cli_b@1 -> sum@1
  }}
  budget {{ steps 100000 }}
  lineage none
}}

---

regulatory {{
  prompts {{ cli_a "a: "  cli_b "b: " }}
  present {{ sum "{{0}} + {{1}} = {{2}}" }}
}}
"#
        );
        let b = format!(
            r#"
body {{
codex 1
# comment
genome {{ cell:{CLI} as cli_b, cli_a
cell:{SUM} as sum }}
wires {{ cli_b@1 -> sum@1
cli_a@1 -> sum@0 }}
grants {{ stdin cli_b cli_a }}
budget {{ steps 100000 }}
lineage none
}}
---
regulatory {{ prompts {{ cli_b "b: " cli_a "x: " }} names {{ sum S }} }}
"#
        );
        // grants list is hashed (G4) — the two spellings above differ in grant
        // order, so they are NOT the same body. A third spelling keeps grant order
        // and only perturbs whitespace, comments, and regulatory names.
        let c = format!(
            r#"
body {{
  genome {{
    cell:{CLI} as cli_a, cli_b
    cell:{SUM} as sum
  }}
  grants {{ stdin: cli_a, cli_b }}
  wires {{ cli_a@1->sum@0 cli_b@1->sum@1 }}
  budget {{ steps 100000 }}
  codex 1
  lineage none
}}
---
regulatory {{ present {{ sum "ignored" }} }}
"#
        );
        let d = format!(
            "body {{ codex 1 genome {{ cell:{SUM} as sum cell:{CLI} as cli_a, cli_b }} grants {{ stdin cli_a cli_b }} wires {{ cli_a@1 -> sum@0 cli_b@1 -> sum@1 }} budget {{ steps 100000 }} lineage none }}\n"
        );
        let pa = print_body(&parse_ok(&a).coding);
        let pc = print_body(&parse_ok(&c).coding);
        let pd = print_body(&parse_ok(&d).coding);
        assert_eq!(
            pa, pc,
            "whitespace/order/regulatory must not move canonical text"
        );
        assert_eq!(pa, pd);
        let pb = print_body(&parse_ok(&b).coding);
        assert_ne!(
            pa, pb,
            "grant order is hashed; swapping cli_a/cli_b must move the body"
        );
        // A fourth spelling: comments and instance display names only.
        let e = format!(
            r#"
# leading comment
body {{
  codex 1
  genome {{ cell:{SUM} as sum cell:{CLI} as cli_a, cli_b }}
  grants {{ stdin: cli_a, cli_b }}
  wires {{ cli_a@1 -> sum@0 cli_b@1 -> sum@1 }}
  budget {{ steps 100000 }}
  lineage none
}}
---
regulatory {{ names {{ cli_a left cli_b right }} }}
"#
        );
        assert_eq!(pa, print_body(&parse_ok(&e).coding));
    }

    #[test]
    fn hand_computed_body_hash_matches() {
        let src = format!(
            "body {{ codex 1 genome {{ cell:{SUM} as sum cell:{CLI} as cli_a, cli_b }} grants {{ stdin cli_a cli_b }} wires {{ cli_a@1 -> sum@0 cli_b@1 -> sum@1 }} budget {{ steps 100000 }} lineage none }}\n"
        );
        let body = parse_ok(&src);
        let text = print_body(&body.coding);
        let h = crate::hash::hash(&body.coding);
        assert_eq!(h, keyed_hash(TAG_BODY, text.as_bytes()));
        // Hand-computed: BLAKE3-256(len_le(tag) ‖ joinn.body.v1 ‖ len_le(bytes) ‖ canonical_text)
        // of the four-spelling calculator body (grant order stdin cli_a cli_b).
        assert_eq!(
            h.to_hex(),
            "b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde"
        );
        let again = crate::hash::hash(&parse_ok(&print_body(&body.coding)).coding);
        assert_eq!(h, again);
    }

    #[test]
    fn unknown_cell_hash_is_refused_by_name() {
        let src = format!(
            "body {{ codex 1 genome {{ cell:{SUM} as sum }} grants {{ }} wires {{ }} budget {{ steps 1 }} lineage none }}\n"
        );
        let body = parse_ok(&src);
        let cells = BTreeMap::new();
        match check_body(&body, &cells) {
            Verdict::Refused(r) => {
                assert!(r.reason.contains(SUM), "{}", r.reason);
            }
            Verdict::Ok(()) => panic!("unknown cell hash must refuse"),
        }
    }

    #[test]
    fn wire_to_nonexistent_port_refuses() {
        use crate::model::sum_cell;
        let src = format!(
            "body {{ codex 1 genome {{ cell:{SUM} as sum }} grants {{ }} wires {{ sum@2 -> sum@9 }} budget {{ steps 1 }} lineage none }}\n"
        );
        let body = parse_ok(&src);
        let cell = sum_cell();
        let mut cells = BTreeMap::new();
        cells.insert(crate::hash::hash(&cell.coding), cell);
        match check_body(&body, &cells) {
            Verdict::Refused(r) => {
                assert!(
                    r.reason.contains("nonexistent") || r.reason.contains("9"),
                    "{}",
                    r.reason
                );
            }
            Verdict::Ok(()) => panic!("wire to missing port must refuse"),
        }
    }

    #[test]
    fn four_prim_spellings_canonicalize_byte_identically() {
        let a = format!(
            r#"
body {{
  codex 1
  genome {{
    prim:case as c
    prim:eq as iseq
    cell:{SUM} as self
  }}
  grants {{ }}
  wires {{ self@1 -> c@0 }}
  budget {{ steps 100000 }}
  lineage none
}}
"#
        );
        let b = format!(
            r#"
body {{
codex 1
genome {{ prim:eq as iseq
prim:case as c
cell:{SUM} as self }}
wires {{ self@1->c@0 }}
grants {{ }}
budget {{ steps 100000 }}
lineage none
}}
"#
        );
        let c = format!(
            "body {{ codex 1 genome {{ cell:{SUM} as self prim:case as c prim:eq as iseq }} grants {{ }} wires {{ self@1 -> c@0 }} budget {{ steps 100000 }} lineage none }}\n"
        );
        let d = format!(
            r#"
# comment
body {{
  genome {{ cell:{SUM} as self prim:eq as iseq prim:case as c }}
  grants {{ }}
  wires {{ self@1 -> c@0 }}
  budget {{ steps 100000 }}
  codex 1
  lineage none
}}
"#
        );
        let pa = print_body(&parse_ok(&a).coding);
        assert_eq!(pa, print_body(&parse_ok(&b).coding));
        assert_eq!(pa, print_body(&parse_ok(&c).coding));
        assert_eq!(pa, print_body(&parse_ok(&d).coding));
        assert!(pa.contains("prim:case as c"));
        assert!(pa.contains("prim:eq as iseq"));
        assert!(pa.contains(&format!("cell:{SUM} as self")));
    }
}
