//! System form: growing bodies, and the forces from outside that reach them.
//! Data only.

mod parse_system;
mod parse_system_regulatory;
mod print_system;
mod system_bodies;
mod system_coding;
mod system_forces;

pub use parse_system::parse_system;
pub use print_system::print_system;

use joinn_frame::{FrameRef, Hash};
use std::collections::BTreeMap;

use crate::body::contact::ForceKind;

/// One bound body: a contact by hash, under an alias.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SystemBody {
    /// The contact's coding hash.
    pub contact: Hash,
    /// The name the system's forces use for it.
    pub alias: String,
}

/// A force on a body. It names no instance: it reaches every grown instance
/// of the body through its receptor.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SystemForce {
    /// Which force.
    pub kind: ForceKind,
    /// The frame it acts on.
    pub frame: FrameRef,
    /// The response law, pinned by coding hash.
    pub response: Hash,
    /// The response's name.
    pub name: String,
    /// The body alias it reaches.
    pub on: String,
}

/// Hashed half of a system.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SystemCoding {
    /// Canonical-form version. Phase 7.4 is 1.
    pub codex: u16,
    /// Bound bodies.
    pub bodies: Vec<SystemBody>,
    /// Forces.
    pub forces: Vec<SystemForce>,
    /// Parent system hash, if any.
    pub lineage: Option<Hash>,
}

/// Never hashed: names, present templates, and the starting shape.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct SystemRegulatory {
    /// Display names.
    pub names: BTreeMap<String, String>,
    /// Response → present template.
    pub present: BTreeMap<String, String>,
    /// Body alias → how many empty boxes the shell shows.
    pub waiting: BTreeMap<String, u32>,
}

/// A system: coding + regulatory.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct System {
    /// Hashed.
    pub coding: SystemCoding,
    /// Never hashed.
    pub regulatory: SystemRegulatory,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::hash;
    use joinn_frame::{TAG_SYSTEM, Verdict, keyed_hash};

    const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
    const A: &str = "1111111111111111111111111111111111111111111111111111111111111111";
    const B: &str = "2222222222222222222222222222222222222222222222222222222222222222";

    fn counting(body: &str, forces: &str, regulatory: &str) -> String {
        format!(
            "system {{\n  codex 1\n  bodies {{\n{body}  }}\n  forces {{\n{forces}  }}\n  lineage none\n}}\n---\nregulatory {{\n{regulatory}}}\n"
        )
    }

    fn standard() -> String {
        counting(
            &format!("    contact:{A} as numbers\n"),
            &format!("    combine ℤ 1 cell:{SUM} as count on numbers\n"),
            "  names   { count \"Count\" numbers \"Numbers\" }\n  present { count \"{0}\" }\n  waiting { numbers 1 }\n",
        )
    }

    fn parse_ok(src: &str) -> System {
        match parse_system(src) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn refused(src: &str) -> String {
        match parse_system(src) {
            Verdict::Refused(r) => r.reason,
            Verdict::Ok(s) => panic!("admitted: {}", print_system(&s.coding)),
        }
    }

    #[test]
    fn a_system_parses_prints_and_hashes_its_canonical_text() {
        let s = parse_ok(&standard());
        let want = format!(
            "codex 1\nbodies\ncontact:{A} as numbers\nforces\ncombine ℤ 1 cell:{SUM} as count on numbers\nlineage none\n"
        );
        assert_eq!(print_system(&s.coding), want);
        assert_eq!(print_system(&parse_ok(&want).coding), want);
        assert_eq!(hash(&s.coding), keyed_hash(TAG_SYSTEM, want.as_bytes()));
        assert_eq!(s.regulatory.waiting.get("numbers"), Some(&1));
        assert_eq!(
            s.regulatory.present.get("count").map(String::as_str),
            Some("{0}")
        );
        let child = standard().replace("lineage none", &format!("lineage {B}"));
        assert_eq!(
            parse_ok(&child).coding.lineage.map(|h| h.to_hex()),
            Some(B.to_owned())
        );
        assert_ne!(hash(&parse_ok(&child).coding), hash(&s.coding));
    }

    #[test]
    fn canonical_order_sorts_forces_by_response() {
        let two = format!(
            "    combine ℤ 1 cell:{SUM} as total on numbers\n    combine ℤ 1 cell:{SUM} as count on numbers\n"
        );
        let swapped = format!(
            "    combine ℤ 1 cell:{SUM} as count on numbers\n    combine ℤ 1 cell:{SUM} as total on numbers\n"
        );
        let body = format!("    contact:{A} as numbers\n");
        let a = parse_ok(&counting(&body, &two, ""));
        let b = parse_ok(&counting(&body, &swapped, ""));
        assert_eq!(print_system(&a.coding), print_system(&b.coding));
        assert_eq!(hash(&a.coding), hash(&b.coding));
        assert!(print_system(&a.coding).contains("as count on numbers\ncombine"));
    }

    #[test]
    fn two_bodies_are_refused() {
        let body = format!("    contact:{A} as numbers\n    contact:{B} as more\n");
        let forces = format!("    combine ℤ 1 cell:{SUM} as count on numbers\n");
        assert_eq!(
            refused(&counting(&body, &forces, "")),
            "system: 2 bodies; acceptance is one body in Phase 7.4"
        );
    }

    #[test]
    fn the_regulatory_region_is_never_hashed() {
        let s = parse_ok(&standard());
        let renamed = parse_ok(
            &standard()
                .replace("count \"Count\"", "count \"Total\"")
                .replace("waiting { numbers 1 }", "waiting { numbers 2 }"),
        );
        assert_ne!(s.regulatory, renamed.regulatory);
        assert_eq!(hash(&s.coding), hash(&renamed.coding));
    }

    #[test]
    fn a_force_that_names_an_instance_is_refused() {
        let body = format!("    contact:{A} as numbers\n");
        let forces = format!("    combine ℤ 1 cell:{SUM} as count from numbers.0\n");
        assert_eq!(
            refused(&counting(&body, &forces, "")),
            "system: force count names no instance (it reaches every grown instance of its body); acceptance is combine <frame> <version> cell:<hash> as <name> on <body alias>"
        );
    }
}
