//! Lay out one lens of a universe on the fixed grid.

use joinn_frame::{Hash, Verdict};
use joinn_link::{BodyStore, Universe, bind};
use std::collections::BTreeMap;

use super::{
    BODIES_MAX, BODY_MAX, Chart, ChartKind, GALAXIES_MAX, GALAXY_SIZE, SYSTEM_SIZE, SYSTEMS_MAX,
    UNIVERSE_SIZE, UniverseLayout,
};
use crate::camera::ChartId;
use crate::layout::layout;
use crate::refuse::refuse;

/// Galaxy `g` at `(64 + 1360·(g mod 4), 64 + 768·(g div 4))`, system `j` of its
/// galaxy at `(16 + 320·(j mod 4), 32 + 168·(j div 4))`, body `i` of its system
/// at `(8 + 48·(i mod 6), 16 + 32·(i div 6))`, each in its parent chart and in
/// canonical (name) order. Bodies bind by hash from `store`.
pub fn layout_universe(
    universe: &Universe,
    store: &BodyStore,
    lens: &str,
) -> Verdict<UniverseLayout> {
    let Some(found) = universe.coding.lenses.iter().find(|l| l.name == lens) else {
        return refuse(format!(
            "layout: lens {lens} not found; acceptance is a lens the universe declares"
        ));
    };
    if found.galaxies.len() > GALAXIES_MAX {
        return refuse(format!(
            "layout: lens {lens} holds {} galaxies; acceptance is at most {GALAXIES_MAX}",
            found.galaxies.len()
        ));
    }
    let bound = match bind(universe, store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Verdict::Refused(r),
    };
    let hash_of: BTreeMap<&str, Hash> = universe
        .coding
        .bodies
        .iter()
        .map(|b| (b.alias.as_str(), b.hash))
        .collect();
    let at = |i: usize, across: usize, x0: i64, dx: i64, y0: i64, dy: i64| {
        let (col, row) = ((i % across) as i64, (i / across) as i64);
        (x0 + dx * col, y0 + dy * row)
    };
    let slot = |n: usize| ChartId(u32::try_from(n).unwrap_or(u32::MAX));

    let mut charts = vec![Chart {
        kind: ChartKind::Universe,
        name: "universe".to_owned(),
        parent: ChartId(0),
        origin: (0, 0),
        root_origin: (0, 0),
        size: UNIVERSE_SIZE,
        index: 0,
        body: None,
    }];
    let mut layouts = Vec::new();
    let mut layout_of: BTreeMap<Hash, usize> = BTreeMap::new();
    let (mut systems_seen, mut bodies_seen) = (0u32, 0u32);
    let mut galaxies = found.galaxies.clone();
    galaxies.sort_by(|a, b| a.name.cmp(&b.name));
    for (g, galaxy) in (0u32..).zip(&galaxies) {
        if galaxy.systems.len() > SYSTEMS_MAX {
            return refuse(format!(
                "layout: galaxy {} holds {} systems; acceptance is at most {SYSTEMS_MAX}",
                galaxy.name,
                galaxy.systems.len()
            ));
        }
        let g_origin = at(g as usize, 4, 64, 1360, 64, 768);
        let g_id = slot(charts.len());
        charts.push(Chart {
            kind: ChartKind::Galaxy,
            name: galaxy.name.clone(),
            parent: ChartId(0),
            origin: g_origin,
            root_origin: g_origin,
            size: GALAXY_SIZE,
            index: g,
            body: None,
        });
        let mut systems = galaxy.systems.clone();
        systems.sort_by(|a, b| a.name.cmp(&b.name));
        for (j, system) in systems.iter().enumerate() {
            if system.bodies.len() > BODIES_MAX {
                return refuse(format!(
                    "layout: system {} holds {} bodies; acceptance is at most {BODIES_MAX}",
                    system.name,
                    system.bodies.len()
                ));
            }
            let s_origin = at(j, 4, 16, 320, 32, 168);
            let s_root = (g_origin.0 + s_origin.0, g_origin.1 + s_origin.1);
            let s_id = slot(charts.len());
            charts.push(Chart {
                kind: ChartKind::System,
                name: system.name.clone(),
                parent: g_id,
                origin: s_origin,
                root_origin: s_root,
                size: SYSTEM_SIZE,
                index: systems_seen,
                body: None,
            });
            systems_seen += 1;
            let mut aliases = system.bodies.clone();
            aliases.sort();
            for (i, alias) in aliases.iter().enumerate() {
                let (Some(hash), Some((body, cells))) =
                    (hash_of.get(alias.as_str()), bound.get(alias))
                else {
                    return refuse(format!(
                        "layout: body {alias} is not bound; acceptance is a body the universe declares"
                    ));
                };
                let li = match layout_of.get(hash) {
                    Some(li) => *li,
                    None => {
                        let l = match layout(body, cells) {
                            Verdict::Ok(l) => l,
                            Verdict::Refused(r) => return Verdict::Refused(r),
                        };
                        layouts.push(l);
                        layout_of.insert(*hash, layouts.len() - 1);
                        layouts.len() - 1
                    }
                };
                let surface = layouts[li].surface;
                if surface.w > BODY_MAX.0 || surface.h > BODY_MAX.1 {
                    return refuse(format!(
                        "layout: body {alias} is {}×{}; acceptance is a surface within 40×24",
                        surface.w, surface.h
                    ));
                }
                let b_origin = at(i, 6, 8, 48, 16, 32);
                charts.push(Chart {
                    kind: ChartKind::Body,
                    name: alias.clone(),
                    parent: s_id,
                    origin: b_origin,
                    root_origin: (s_root.0 + b_origin.0, s_root.1 + b_origin.1),
                    size: (surface.w, surface.h),
                    index: bodies_seen,
                    body: Some((*hash, li)),
                });
                bodies_seen += 1;
            }
        }
    }
    Verdict::Ok(UniverseLayout {
        lens: lens.to_owned(),
        charts,
        layouts,
    })
}

#[cfg(test)]
mod tests {
    use joinn_dna::{hash, parse_body};
    use joinn_frame::{FrameRegistry, Verdict};
    use joinn_link::{BodyStore, Universe, parse_universe};

    use super::layout_universe;
    use crate::charts::ChartKind;
    use crate::fixtures::calculator;

    fn store() -> BodyStore {
        let (body, cells) = calculator();
        let mut store = BodyStore::new();
        let mut all = cells.clone();
        for src in [include_str!("../../../../corpus/phase21/mul.cell")] {
            if let Verdict::Ok(c) = joinn_dna::parse_cell(src, &FrameRegistry::phase1()) {
                all.insert(hash(&c.coding), c);
            }
        }
        if let Verdict::Refused(r) = store.insert(body, all.clone(), "calculator.body") {
            panic!("{}", r.reason);
        }
        let units = match parse_body(
            include_str!("../../../../corpus/phase5/units.body"),
            &FrameRegistry::phase1(),
        ) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        if let Verdict::Refused(r) = store.insert(units, all, "units.body") {
            panic!("{}", r.reason);
        }
        store
    }

    fn universe() -> Universe {
        match parse_universe(include_str!("../../../../corpus/phase5/universe.universe")) {
            Verdict::Ok(u) => u,
            Verdict::Refused(r) => panic!("{}", r.reason),
        }
    }

    #[test]
    fn the_phase_5_universe_lays_out_its_function_lens_on_the_grid() {
        let l = match layout_universe(&universe(), &store(), "function") {
            Verdict::Ok(l) => l,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        type Row<'a> = (ChartKind, &'a str, (i64, i64), (i64, i64), (i64, i64));
        let got: Vec<Row> = l
            .charts
            .iter()
            .map(|c| (c.kind, c.name.as_str(), c.origin, c.root_origin, c.size))
            .collect();
        assert_eq!(
            got,
            [
                (
                    ChartKind::Universe,
                    "universe",
                    (0, 0),
                    (0, 0),
                    (5504, 1600)
                ),
                (ChartKind::Galaxy, "app", (64, 64), (64, 64), (1296, 704)),
                (
                    ChartKind::System,
                    "calculation",
                    (16, 32),
                    (80, 96),
                    (304, 152)
                ),
                (ChartKind::Body, "calc", (8, 16), (88, 112), (40, 24)),
                (
                    ChartKind::System,
                    "measurement",
                    (336, 32),
                    (400, 96),
                    (304, 152)
                ),
                (ChartKind::Body, "units", (8, 16), (408, 112), (20, 18)),
            ]
        );
        assert_eq!(l.layouts.len(), 2);
    }

    #[test]
    fn a_lens_the_universe_does_not_declare_is_refused() {
        let Verdict::Refused(r) = layout_universe(&universe(), &store(), "nowhere") else {
            panic!("lens nowhere must be refused");
        };
        assert_eq!(
            r.reason,
            "layout: lens nowhere not found; acceptance is a lens the universe declares"
        );
    }

    #[test]
    fn a_system_of_25_bodies_is_refused_naming_it_and_the_count() {
        let mut u = universe();
        let lens = &mut u.coding.lenses[1];
        assert_eq!(lens.name, "function");
        let system = &mut lens.galaxies[0].systems[0];
        system.bodies = (0..25).map(|_| "calc".to_owned()).collect();
        let Verdict::Refused(r) = layout_universe(&u, &store(), "function") else {
            panic!("25 bodies must be refused");
        };
        assert_eq!(
            r.reason,
            "layout: system calculation holds 25 bodies; acceptance is at most 24"
        );
    }

    #[test]
    fn a_galaxy_of_17_systems_and_a_lens_of_9_galaxies_are_refused() {
        let mut u = universe();
        let galaxy = &mut u.coding.lenses[1].galaxies[0];
        let one = galaxy.systems[0].clone();
        galaxy.systems = (0..17).map(|_| one.clone()).collect();
        let Verdict::Refused(r) = layout_universe(&u, &store(), "function") else {
            panic!("17 systems must be refused");
        };
        assert_eq!(
            r.reason,
            "layout: galaxy app holds 17 systems; acceptance is at most 16"
        );
        let mut u = universe();
        let lens = &mut u.coding.lenses[1];
        let one = lens.galaxies[0].clone();
        lens.galaxies = (0..9).map(|_| one.clone()).collect();
        let Verdict::Refused(r) = layout_universe(&u, &store(), "function") else {
            panic!("9 galaxies must be refused");
        };
        assert_eq!(
            r.reason,
            "layout: lens function holds 9 galaxies; acceptance is at most 8"
        );
    }

    #[test]
    fn a_body_wider_than_40_is_refused_naming_it() {
        let wide = "body {\n  codex 1\n  genome {\n    prim:eq as a, b, c\n  }\n  grants { }\n  wires {\n    a@0 -> b@0\n    b@0 -> c@0\n  }\n  budget { steps 1 }\n  lineage none\n}\n";
        let body = match parse_body(wide, &FrameRegistry::phase1()) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        let mut s = store();
        let h = hash(&body.coding);
        if let Verdict::Refused(r) = s.insert(body, std::collections::BTreeMap::new(), "wide") {
            panic!("{}", r.reason);
        }
        let mut u = universe();
        u.coding.bodies[0].hash = h;
        let Verdict::Refused(r) = layout_universe(&u, &s, "function") else {
            panic!("a 3-column body must be refused");
        };
        assert_eq!(
            r.reason,
            "layout: body calc is 60×18; acceptance is a surface within 40×24"
        );
    }
}
