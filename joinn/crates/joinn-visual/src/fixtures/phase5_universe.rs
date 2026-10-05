//! Test fixture: the phase 5 universe's `function` lens, laid out.

use joinn_dna::{hash, parse_body, parse_cell};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_link::{BodyStore, parse_universe};

use super::calculator;
use crate::charts::{UniverseLayout, layout_universe};

/// `phase5/universe.universe`'s `function` lens, binding the calculator and
/// `phase5/units.body` (with `phase21/mul.cell`).
pub(crate) fn phase5_universe() -> UniverseLayout {
    let (body, mut cells) = calculator();
    if let Verdict::Ok(c) = parse_cell(
        include_str!("../../../../corpus/phase21/mul.cell"),
        &FrameRegistry::phase1(),
    ) {
        cells.insert(hash(&c.coding), c);
    }
    let units = match parse_body(
        include_str!("../../../../corpus/phase5/units.body"),
        &FrameRegistry::phase1(),
    ) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut store = BodyStore::new();
    for (b, src) in [(body, "calculator.body"), (units, "units.body")] {
        if let Verdict::Refused(r) = store.insert(b, cells.clone(), src) {
            panic!("{}", r.reason);
        }
    }
    let u = match parse_universe(include_str!("../../../../corpus/phase5/universe.universe")) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match layout_universe(&u, &store, "function") {
        Verdict::Ok(l) => l,
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
}
