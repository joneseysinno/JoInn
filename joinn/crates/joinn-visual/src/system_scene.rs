//! The system scene: a growing body, its waiting boxes, the lasso and the
//! count, drawn as tables (plan 7.4 §2.8). A growth step is a delta; regrow
//! from the system and the whole input list must equal it byte for byte.

mod allows;
mod apply_growth;
mod fit;
mod grow;
mod print_id;
mod regrow;
mod shapes;
mod take_pending;
mod write_rows;

use std::collections::{BTreeMap, BTreeSet};

use joinn_dna::{Cell, Contact};
use joinn_frame::{Hash, Value};
use joinn_link::Grown;

use crate::system_layout::SystemLayout;
use crate::tables::{RowWrite, Tables};

/// A system drawn as tables.
#[derive(Clone, Debug)]
pub struct SystemScene {
    contacts: BTreeMap<Hash, Contact>,
    cells: BTreeMap<Hash, Cell>,
    waiting: u32,
    grown: Grown,
    count: Value,
    layout: SystemLayout,
    tables: Tables,
    pending: BTreeSet<RowWrite>,
}

impl SystemScene {
    /// The tables as they stand.
    pub fn tables(&self) -> &Tables {
        &self.tables
    }

    /// The layout the tables were written from.
    pub fn layout(&self) -> &SystemLayout {
        &self.layout
    }

    /// The system and the inputs it accepted.
    pub fn grown(&self) -> &Grown {
        &self.grown
    }

    /// The response, as the engine gives it.
    pub fn count(&self) -> &Value {
        &self.count
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use joinn_dna::{Cell, Contact, System, hash, parse_cell, parse_contact, parse_system};
    use joinn_frame::{Frame, FrameRegistry, Hash, IntFrame, Term, Value, Verdict};

    use super::SystemScene;
    use crate::pick::{FORCE_TAG, PORT_TAG, Pick, cpu_pick};
    use crate::tables::{RowWrite, STYLE_FORCE, Table, table_bytes};

    fn int(n: i64) -> Value {
        match IntFrame::new().canonicalize(Term::int(n)) {
            Verdict::Ok(v) => v,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn cells() -> BTreeMap<Hash, Cell> {
        let mut out = BTreeMap::new();
        for src in [
            include_str!("../../../corpus/phase0/sum.cell"),
            include_str!("../../../corpus/phase0/cli_input.cell"),
            include_str!("../../../corpus/phase0/format.cell"),
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

    fn contacts() -> BTreeMap<Hash, Contact> {
        [
            include_str!("../../../corpus/phase74/counting.contact"),
            include_str!("../../../corpus/phase74/adding.contact"),
        ]
        .into_iter()
        .map(|src| match parse_contact(src, &FrameRegistry::phase1()) {
            Verdict::Ok(c) => (hash(&c.coding), c),
            Verdict::Refused(r) => panic!("{}", r.reason),
        })
        .collect()
    }

    fn system(src: &str) -> System {
        match parse_system(src) {
            Verdict::Ok(s) => s,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    fn scene(s: &System, waiting: u32) -> SystemScene {
        match SystemScene::grow(s, &contacts(), &cells(), waiting) {
            Verdict::Ok(sc) => sc,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn a_growth_step_writes_only_the_rows_it_changes() {
        let counting = system(include_str!("../../../corpus/phase74/counting.system"));
        let mut sc = scene(&counting, 1);
        assert_eq!(sc.count().print_term(), "0");
        assert_eq!(
            sc.tables().stroke[..11]
                .iter()
                .filter(|s| s.style == STYLE_FORCE)
                .count(),
            11
        );
        for _ in 0..7 {
            if let Verdict::Refused(r) = sc.apply_growth(&int(1)) {
                panic!("{}", r.reason);
            }
        }
        sc.take_pending();
        let before = sc.tables().clone();
        let delta = match sc.apply_growth(&int(1)) {
            Verdict::Ok(d) => d,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(sc.count().print_term(), "8");
        let written = |table: Table, slot: u32| delta.rows.contains(&RowWrite { table, slot });
        assert!(
            !written(Table::Body, 0),
            "the surface keeps its size inside a row"
        );
        assert!(
            written(Table::Cell, 8) && written(Table::Cell, 9),
            "numbers.7 fills, numbers.8 waits"
        );
        for slot in 0..8 {
            assert!(!written(Table::Cell, slot), "cell {slot}");
            assert!(!written(Table::Port, slot), "port {slot}");
        }
        assert!(written(Table::Port, 8), "numbers.7's in-port fills");
        assert!(
            (0..11).all(|slot| !written(Table::Stroke, slot)),
            "the lasso stays"
        );
        assert_eq!(table_bytes(sc.tables()).body, table_bytes(&before).body);
        assert_eq!(sc.take_pending(), Some(delta));
        let kept = sc.tables().clone();
        match sc.apply_growth(&int(3)) {
            Verdict::Refused(r) => assert_eq!(
                r.reason,
                "counting: 3 is not one; acceptance is 1 (counting grows by one)"
            ),
            Verdict::Ok(_) => panic!("counting accepted 3"),
        }
        assert_eq!(sc.tables(), &kept, "a refusal writes nothing");
        assert_eq!(sc.take_pending(), None);
    }

    #[test]
    fn the_lasso_owns_pixels_under_the_force_and_every_allowed_owner_shows() {
        let counting = system(include_str!("../../../corpus/phase74/counting.system"));
        let sc = match SystemScene::regrow(
            &counting,
            &contacts(),
            &cells(),
            1,
            &[int(1), int(1), int(1)],
        ) {
            Verdict::Ok(sc) => sc,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let camera = sc.fit(1280, 720);
        let picked = match cpu_pick(&sc.shapes(), &camera) {
            Verdict::Ok(p) => p,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let owners: BTreeSet<[u32; 4]> = picked
            .pixels
            .iter()
            .filter_map(|p| match p {
                Pick::Owned(id) => Some(*id),
                _ => None,
            })
            .collect();
        let force = [1, 0, FORCE_TAG, 1];
        assert!(owners.contains(&force), "the lasso owns pixels");
        assert_eq!(
            owners,
            sc.allows(),
            "every owner the cut allows shows, and no other"
        );
        assert_eq!(sc.print_id(force), "force count");
        assert_eq!(sc.print_id([1, 1, 0, 1]), "count");
        assert_eq!(sc.print_id([1, 4, PORT_TAG, 1]), "numbers.2@0");
        assert_eq!(sc.print_id([1, 5, 0, 1]), "numbers.3");
        assert_eq!(sc.print_id([1, 0, 0, 1]), "surface");
    }

    #[test]
    fn tables_after_seven_deltas_equal_regrow() {
        for (src, waiting, inputs) in [
            (
                include_str!("../../../corpus/phase74/counting.system"),
                1,
                [1, 1, 1, 1, 1, 1, 1],
            ),
            (
                include_str!("../../../corpus/phase74/adding.system"),
                2,
                [2, 3, 4, -2, 0, 7, 12],
            ),
        ] {
            let s = system(src);
            let mut sc = scene(&s, waiting);
            let mut so_far = Vec::new();
            for v in inputs {
                if let Verdict::Refused(r) = sc.apply_growth(&int(v)) {
                    panic!("{}", r.reason);
                }
                so_far.push(int(v));
                let fresh = match SystemScene::regrow(&s, &contacts(), &cells(), waiting, &so_far) {
                    Verdict::Ok(f) => f,
                    Verdict::Refused(r) => panic!("{}", r.reason),
                };
                assert_eq!(
                    table_bytes(sc.tables()),
                    table_bytes(fresh.tables()),
                    "n {}",
                    so_far.len()
                );
                assert_eq!(sc.count(), fresh.count());
            }
        }
    }
}
