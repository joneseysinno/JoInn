//! Evolution, checked: every old witness holds, and something is gained.

use joinn_dna::{Cell, Contact, System, hash};
use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, Value, Verdict};
use std::collections::BTreeMap;

use super::accept_word::accept_word;
use super::transcripts::transcripts;
use super::{Grown, grow, grow_step, respond};

/// What an evolution kept and what it gained.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Evolution {
    /// Parent transcripts accepted whole, on which the child agreed.
    pub witnesses: usize,
    /// The first transcript input the parent refuses and the child accepts.
    pub gained: Value,
}

/// The child names the parent in `lineage`, and:
/// 1. on each parent transcript, the child accepts every input the parent
///    accepts and gives the same response after every step (a transcript the
///    parent accepts whole is a witness);
/// 2. some transcript input the parent refuses, the child accepts.
pub fn check_evolution(
    parent: &System,
    child: &System,
    contacts: &BTreeMap<Hash, Contact>,
    cells: &BTreeMap<Hash, Cell>,
) -> Verdict<Evolution> {
    let parent_hash = hash(&parent.coding);
    if child.coding.lineage != Some(parent_hash) {
        let named = child
            .coding
            .lineage
            .map_or_else(|| "none".to_string(), |h| h.to_hex());
        return crate::refuse(format!(
            "evolution: the child's lineage is {named}; acceptance is the parent's hash {parent_hash}"
        ));
    }
    let accepts = |s: &System| {
        s.coding
            .bodies
            .first()
            .and_then(|b| contacts.get(&b.contact))
            .and_then(|c| c.coding.grows.as_ref())
            .map(|g| g.accepts)
    };
    let (Some(pa), Some(ca)) = (accepts(parent), accepts(child)) else {
        return crate::refuse(
            "evolution: parent and child each need a growing body; acceptance is a contact with a grows section",
        );
    };
    let (pw, cw) = (accept_word(pa), accept_word(ca));
    let (empty_parent, empty_child) = match (
        grow(parent, contacts, cells, &[]),
        grow(child, contacts, cells, &[]),
    ) {
        (Verdict::Ok(p), Verdict::Ok(c)) => (p, c),
        (Verdict::Refused(r), _) | (_, Verdict::Refused(r)) => return Verdict::Refused(r),
    };
    let frames = FrameRegistry::phase1();
    let agree = |p: &Grown, c: &Grown| -> Verdict<()> {
        match (respond(p, cells, &frames), respond(c, cells, &frames)) {
            (Verdict::Ok(x), Verdict::Ok(y)) if x == y => Verdict::Ok(()),
            (Verdict::Ok(x), Verdict::Ok(y)) => {
                let inputs: Vec<String> = p.inputs().iter().map(Value::print_term).collect();
                crate::refuse(format!(
                    "evolution: after [{}] {cw} gives {} where {pw} gives {}; acceptance is every old witness",
                    inputs.join(" "),
                    y.print_term(),
                    x.print_term()
                ))
            }
            (Verdict::Refused(r), _) | (_, Verdict::Refused(r)) => Verdict::Refused(r),
        }
    };
    let int = |v: i64| IntFrame::new().canonicalize(Term::int(v));
    let mut witnesses = 0;
    for transcript in transcripts(pa) {
        let (mut p, mut c) = (empty_parent.clone(), empty_child.clone());
        if let Verdict::Refused(r) = agree(&p, &c) {
            return Verdict::Refused(r);
        }
        let mut whole = true;
        for v in transcript {
            let v = match int(v) {
                Verdict::Ok(v) => v,
                Verdict::Refused(r) => return Verdict::Refused(r),
            };
            p = match grow_step(&p, &v) {
                Verdict::Ok(g) => g,
                Verdict::Refused(_) => {
                    whole = false;
                    break;
                }
            };
            c = match grow_step(&c, &v) {
                Verdict::Ok(g) => g,
                Verdict::Refused(r) => {
                    return crate::refuse(format!(
                        "evolution: {cw} refuses {} where {pw} accepts it ({}); acceptance is every old witness",
                        v.print_term(),
                        r.reason
                    ));
                }
            };
            if let Verdict::Refused(r) = agree(&p, &c) {
                return Verdict::Refused(r);
            }
        }
        if whole {
            witnesses += 1;
        }
    }
    let mut probes: Vec<i64> = Vec::new();
    for v in transcripts(pa).into_iter().chain(transcripts(ca)).flatten() {
        if !probes.contains(&v) {
            probes.push(v);
        }
    }
    for v in probes {
        let v = match int(v) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => return Verdict::Refused(r),
        };
        let parent_refuses = matches!(grow_step(&empty_parent, &v), Verdict::Refused(_));
        let child_accepts = matches!(grow_step(&empty_child, &v), Verdict::Ok(_));
        if parent_refuses && child_accepts {
            return Verdict::Ok(Evolution {
                witnesses,
                gained: v,
            });
        }
    }
    crate::refuse(format!(
        "evolution: {cw} accepts nothing {pw} refuses; acceptance is a new ability (otherwise it is an edit)"
    ))
}
