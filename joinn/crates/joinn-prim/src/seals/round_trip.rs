//! parse(format n) round-trip check.

use crate::alleles::ParseText;
use joinn_dna::{Body, Cell};
use joinn_frame::{Frame, Hash, Term, TextFrame, Verdict};
use joinn_gate::Oracle;
use std::collections::BTreeMap;

use super::bounded_int::bounded_int;
use super::refuse::refuse;
use super::truth_violation::truth_violation;
use super::{DnaFire, ROUNDTRIP_SAMPLES, Seal};

pub(in crate::seals) fn round_trip(
    dna: &dyn DnaFire,
    bodies: &BTreeMap<Hash, Body>,
    cells: &BTreeMap<Hash, Cell>,
    seals: &[Seal],
    seed: u64,
) -> Verdict<Vec<String>> {
    let Some(parse) = seals.iter().find(|s| s.sealed.0 == "parse@Text") else {
        return Verdict::Ok(Vec::new());
    };
    let Some(format) = seals.iter().find(|s| s.sealed.0 == "format@ℤ") else {
        return Verdict::Ok(Vec::new());
    };
    let Some(parse_cell) = cells.get(&parse.cell) else {
        return Verdict::Refused(refuse("round-trip: parse cell missing"));
    };
    let Some(format_cell) = cells.get(&format.cell) else {
        return Verdict::Refused(refuse("round-trip: format cell missing"));
    };
    let Some(parse_body) = bodies.get(&parse.reference.hash) else {
        return Verdict::Refused(refuse("round-trip: parse body missing"));
    };
    let Some(format_body) = bodies.get(&format.reference.hash) else {
        return Verdict::Refused(refuse("round-trip: format body missing"));
    };
    let hole = match TextFrame::new().canonicalize(Term::text("007")) {
        Verdict::Ok(v) => v,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let hole_parse = ParseText.apply(&BTreeMap::from([(0, hole)]));
    if !matches!(hole_parse, Verdict::Refused(_)) {
        return Verdict::Refused(refuse(
            "round-trip hole \"007\": parse@Text accepted leading zeros",
        ));
    }
    for i in 0..ROUNDTRIP_SAMPLES {
        let n = bounded_int(seed, i, format.drives.bound);
        let fmt = match dna.fire(
            format_body,
            format_cell,
            cells,
            bodies,
            &BTreeMap::from([(0, n.clone())]),
        ) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => {
                return Verdict::Refused(truth_violation(
                    "int.format",
                    seed.wrapping_add(u64::from(i)),
                    &BTreeMap::from([(0, n)]),
                    None,
                    None,
                    &format!("format refused in round-trip ({})", r.reason),
                ));
            }
        };
        let Some(text) = fmt.get(&1).cloned() else {
            return Verdict::Refused(refuse("round-trip: format produced no text"));
        };
        let parsed = match dna.fire(
            parse_body,
            parse_cell,
            cells,
            bodies,
            &BTreeMap::from([(0, text)]),
        ) {
            Verdict::Ok(m) => m,
            Verdict::Refused(r) => {
                return Verdict::Refused(truth_violation(
                    "text.parse_int",
                    seed.wrapping_add(u64::from(i)),
                    &BTreeMap::from([(0, n)]),
                    None,
                    None,
                    &format!("parse refused in round-trip ({})", r.reason),
                ));
            }
        };
        let got = parsed.get(&1);
        if got != Some(&n) {
            if !format.one_way && !parse.one_way {
                return Verdict::Refused(truth_violation(
                    "parse∘format",
                    seed.wrapping_add(u64::from(i)),
                    &BTreeMap::from([(0, n.clone())]),
                    Some(n),
                    got.cloned(),
                    "lossy pair declared one_way: false",
                ));
            }
            return Verdict::Refused(truth_violation(
                "parse∘format",
                seed.wrapping_add(u64::from(i)),
                &BTreeMap::from([(0, n.clone())]),
                Some(n),
                got.cloned(),
                "parse(format n) ≠ n",
            ));
        }
    }
    if !parse.one_way {
        return Verdict::Refused(refuse(
            "round-trip: parse seal is not declared one_way but \"007\" is a hole",
        ));
    }
    Verdict::Ok(vec![format!(
        "round-trip: parse(format n) = n on {ROUNDTRIP_SAMPLES} samples; hole \"007\" named (one_way read from the parse seal)"
    )])
}
