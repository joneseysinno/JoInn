//! Contact form: cells in contact, and the forces that act on them. A contact
//! has no wires: the engine derives its own deliveries. Data only.

#![allow(clippy::result_large_err)]

mod contact_coding;
mod contact_forces;
mod contact_genome;
mod contact_grants;
mod contact_members;
mod contact_names;
mod is_contact_section;
mod parse_contact;
mod print_contact;

pub use parse_contact::parse_contact;
pub use print_contact::print_contact;

use crate::body::BodyRegulatory;
use joinn_frame::{FrameRef, Hash};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// A force. Closed: Phase 7 writes combine only.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ForceKind {
    /// Cells brought together; the response appears beside them.
    Combine,
}

impl ForceKind {
    /// The word a `.contact` file writes.
    pub fn word(self) -> &'static str {
        match self {
            ForceKind::Combine => "combine",
        }
    }
}

/// One port a force reaches: `instance@port`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Member {
    /// Genome instance.
    pub instance: String,
    /// Port position on that instance.
    pub port: u32,
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.instance, self.port)
    }
}

/// A force from outside: its frame, its pinned response, and what it reaches.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Force {
    /// Which force.
    pub kind: ForceKind,
    /// The frame it acts on.
    pub frame: FrameRef,
    /// The response law, pinned by coding hash.
    pub response: Hash,
    /// The response's instance name.
    pub name: String,
    /// Ports reached. Order is never hashed.
    pub members: Vec<Member>,
}

/// What a growing body accepts. Closed: Phase 7.4 writes one and any.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Accept {
    /// Exactly `1` in ℤ (counting).
    One,
    /// Any value in ℤ (adding).
    Any,
}

impl Accept {
    /// The word a `.contact` file writes.
    pub fn word(self) -> &'static str {
        match self {
            Accept::One => "one",
            Accept::Any => "any",
        }
    }
}

/// How a body grows: the cell each growth adds, the growth name its grown
/// instances carry (`<name>.0`, `<name>.1`, …), and what it accepts.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grows {
    /// The cell each growth adds.
    pub cell: Hash,
    /// The growth name.
    pub name: String,
    /// What an input must be.
    pub accepts: Accept,
}

/// One genome entry. A contact's genome holds cells only.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CellEntry {
    /// Full 64-hex cell identity.
    pub cell: Hash,
    /// Instance names.
    pub instances: Vec<String>,
}

/// Hashed half of a contact.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ContactCoding {
    /// Canonical-form version. Phase 7 is 1.
    pub codex: u16,
    /// Cells in this body.
    pub genome: Vec<CellEntry>,
    /// Capability name → instance names, in grant order.
    pub grants: BTreeMap<String, Vec<String>>,
    /// Environment signals this body reads. Omitted from print when empty.
    pub reads: BTreeSet<String>,
    /// Forces.
    pub forces: Vec<Force>,
    /// How the body grows, if it does. Printed only when present.
    pub grows: Option<Grows>,
    /// Step budget.
    pub budget_steps: u64,
    /// Parent body hash, if any.
    pub lineage: Option<Hash>,
}

/// A contact body: coding + regulatory.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Contact {
    /// Hashed.
    pub coding: ContactCoding,
    /// Never hashed. A response name may appear here like any instance.
    pub regulatory: BodyRegulatory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::hash;
    use joinn_frame::{FrameRegistry, TAG_CONTACT, Verdict, keyed_hash};

    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";
    const CORPUS: &str = include_str!("../../../../corpus/phase7/calculator.contact");

    fn parse_ok(src: &str) -> Contact {
        match parse_contact(src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => c,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn refused(src: &str) -> String {
        match parse_contact(src, &FrameRegistry::phase1()) {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(c) => panic!("admitted: {}", print_contact(&c.coding)),
        }
    }

    fn with_coding(inner: &str) -> String {
        format!("contact {{ codex 1 {inner} budget {{ steps 100000 }} lineage none }}\n")
    }

    #[test]
    fn four_spellings_share_one_hash_and_grant_order_moves_it() {
        let b = format!(
            r#"
# members reversed, sections reordered
contact {{
  lineage none
  budget {{ steps 100000 }}   # the budget first
  forces {{
    combine ℤ 1 cell:{SUM} as sum from cli_b@1 cli_a@1
  }}
  grants {{ stdin: cli_a, cli_b }}
  genome {{ cell:{CLI} as cli_b, cli_a }}
  codex 1
}}
---
regulatory {{
  prompts {{ cli_a "a: "  cli_b "b: " }}
  present {{ sum "{{0}} + {{1}} = {{2}}" }}
}}
"#
        );
        let c = format!(
            "contact {{ codex 1 genome {{ cell:{CLI} as cli_a, cli_b }} grants {{ stdin cli_a cli_b }} forces {{ combine ℤ 1 cell:{SUM} as sum from cli_a@1, cli_b@1 }} budget {{ steps 100000 }} lineage none }}\n"
        );
        let d = CORPUS.replace(r#"labels { sum "Sum" }"#, r#"labels { sum "Total" }"#);
        assert_ne!(d, CORPUS);
        let want = format!(
            "budget\nsteps 100000\ncodex 1\ngenome\ncell:{CLI} as cli_a, cli_b\ngrants\nstdin cli_a cli_b\nlineage none\nforces\ncombine ℤ 1 cell:{SUM} as sum from cli_a@1 cli_b@1\n"
        );
        let base = parse_ok(CORPUS);
        assert_eq!(print_contact(&base.coding), want);
        for src in [&b, &c, &d] {
            let other = parse_ok(src);
            assert_eq!(print_contact(&other.coding), want, "{src}");
            assert_eq!(hash(&other.coding), hash(&base.coding), "{src}");
        }
        assert_eq!(hash(&base.coding), keyed_hash(TAG_CONTACT, want.as_bytes()));
        assert_eq!(
            hash(&base.coding).to_hex(),
            "868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209"
        );
        let swapped = CORPUS.replace("stdin: cli_a, cli_b", "stdin: cli_b, cli_a");
        assert_ne!(
            hash(&parse_ok(&swapped).coding),
            hash(&base.coding),
            "grant order is hashed (G4)"
        );
    }

    #[test]
    fn the_regulatory_region_is_a_bodys() {
        let c = parse_ok(CORPUS);
        assert_eq!(
            c.regulatory.prompts.get("cli_a").map(String::as_str),
            Some("a: ")
        );
        assert_eq!(
            c.regulatory.present.get("sum").map(String::as_str),
            Some("{0} + {1} = {2}")
        );
        assert_eq!(
            c.regulatory.labels.get("sum").map(String::as_str),
            Some("Sum")
        );
    }

    #[test]
    fn the_canonical_text_reparses_to_itself() {
        let printed = print_contact(&parse_ok(CORPUS).coding);
        assert_eq!(print_contact(&parse_ok(&printed).coding), printed);
    }

    #[test]
    fn a_wires_section_is_refused() {
        let src = CORPUS.replace("  budget {", "  wires { cli_a@1 -> sum@0 }\n  budget {");
        assert_eq!(
            refused(&src),
            "contact: wires belong to systems; a body's cells touch. acceptance is a forces section"
        );
    }

    #[test]
    fn a_force_word_other_than_combine_is_refused() {
        let src = CORPUS.replace("combine ℤ 1", "separate ℤ 1");
        assert_eq!(
            refused(&src),
            "contact codex 1 writes combine only; separate is not written here (separate is combine read at a turn). acceptance is combine"
        );
    }

    #[test]
    fn a_prim_genome_entry_is_refused() {
        let src = with_coding("genome { prim:eq as iseq }");
        assert_eq!(
            refused(&src),
            "contact: a genome entry is a cell; prim:eq belongs to .body. acceptance is cell:<hash>"
        );
    }

    #[test]
    fn a_declarations_section_is_refused() {
        let src = with_coding("declarations { assert H₁ = 0 }");
        assert_eq!(
            refused(&src),
            "contact codex 1 has no declarations; acceptance is a .body or .universe for an assert"
        );
    }

    #[test]
    fn a_response_named_like_a_cell_is_refused() {
        let src = CORPUS.replace("as sum from", "as cli_b from");
        assert_eq!(
            refused(&src),
            "contact: cli_b is both a cell and a force's response; acceptance is a new name"
        );
    }

    #[test]
    fn a_member_written_twice_is_refused() {
        let src = CORPUS.replace("from cli_a@1, cli_b@1", "from cli_a@1, cli_a@1");
        assert_eq!(
            refused(&src),
            "contact: cli_a@1 is written twice in force sum"
        );
    }
}
