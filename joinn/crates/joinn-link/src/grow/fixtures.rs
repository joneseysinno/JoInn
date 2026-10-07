//! Hand-written counting and adding systems for the growth tests.
// allow(modules): shared test helpers — not production operations

use joinn_dna::{Cell, Contact, System, hash, parse_cell, parse_contact, parse_system};
use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, Value, Verdict};
use std::collections::BTreeMap;

use super::{grow, respond};

const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";
const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";

pub(crate) fn int(n: i64) -> Value {
    match IntFrame::new().canonicalize(Term::int(n)) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}

pub(crate) fn cells() -> BTreeMap<Hash, Cell> {
    let mut out = BTreeMap::new();
    for src in [
        include_str!("../../../../corpus/phase0/sum.cell"),
        include_str!("../../../../corpus/phase0/cli_input.cell"),
        include_str!("../../../../corpus/phase0/format.cell"),
    ] {
        match parse_cell(src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => {
                out.insert(hash(&c.coding), c);
            }
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }
    out
}

fn system(accepts: &str) -> (System, BTreeMap<Hash, Contact>) {
    descendant(accepts, None)
}

/// A system whose body accepts so; with a parent, both the contact and the
/// system name the parent's as their lineage.
pub(crate) fn descendant(
    accepts: &str,
    parent: Option<(&System, &BTreeMap<Hash, Contact>)>,
) -> (System, BTreeMap<Hash, Contact>) {
    let lineage = |h: Option<Hash>| h.map_or_else(|| "none".to_string(), |h| h.to_hex());
    let parent_contact = parent.and_then(|(_, cs)| cs.keys().next().copied());
    let parent_system = parent.map(|(s, _)| hash(&s.coding));
    let src = format!(
        "contact {{ codex 1 grows {{ cell:{CLI} as numbers accepts {accepts} }} budget {{ steps 100000 }} lineage {} }}\n",
        lineage(parent_contact)
    );
    let contact = match parse_contact(&src, &FrameRegistry::phase1()) {
        Verdict::Ok(c) => c,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let src = format!(
        "system {{ codex 1 bodies {{ contact:{} as numbers }} forces {{ combine ℤ 1 cell:{SUM} as count on numbers }} lineage {} }}\n",
        hash(&contact.coding),
        lineage(parent_system)
    );
    let system = match parse_system(&src) {
        Verdict::Ok(s) => s,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    (system, BTreeMap::from([(hash(&contact.coding), contact)]))
}

pub(crate) fn counting() -> (System, BTreeMap<Hash, Contact>) {
    system("one")
}

pub(crate) fn adding() -> (System, BTreeMap<Hash, Contact>) {
    system("any")
}

pub(crate) fn run(system: &System, contacts: &BTreeMap<Hash, Contact>, inputs: &[Value]) -> Value {
    let cells = cells();
    let grown = match grow(system, contacts, &cells, inputs) {
        Verdict::Ok(g) => g,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match respond(&grown, &cells, &FrameRegistry::phase1()) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}
