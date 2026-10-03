//! Grow the grove from its seed: bodies, links and the one lens.

use joinn_frame::Hash;
use joinn_link::{
    BodyBinding, Galaxy, Lens, Link, Mark, Member, Order, System, Universe, UniverseCoding,
    UniverseRegulatory,
};
use std::collections::BTreeMap;

use super::{GALAXIES, GroveKind, KINDS, SLOTS, SYSTEMS, splitmix64};

/// Slots 0–2 are calc, 3–4 units, 5 bus; slots 6–23 draw their kind from
/// SplitMix64 in (galaxy, system, slot) order. Links `sys_`, `gal_` and `uni`
/// are `order none`, tail first. One lens, `function`. No grants, no
/// regulatory region.
pub(crate) fn grow_grove(seed: u64) -> Result<Universe, String> {
    let mut hashes = BTreeMap::new();
    for kind in KINDS {
        let Some(h) = Hash::parse_hex(kind.hash_hex()) else {
            return Err(format!("grove: {} hash does not parse", kind.name()));
        };
        hashes.insert(kind, h);
    }
    let alias =
        |g: usize, s: usize, slot: usize| format!("b{:04}", (g * SYSTEMS + s) * SLOTS + slot);
    let member = |body: String, instance: &str, port: u32, mark: Mark| Member {
        body,
        instance: instance.to_owned(),
        port,
        mark,
    };

    let mut state = seed;
    let mut bodies = Vec::new();
    let mut galaxies = Vec::new();
    let mut links = Vec::new();
    for g in 0..GALAXIES {
        let mut systems = Vec::new();
        for s in 0..SYSTEMS {
            let name = format!("g{g}s{s:02}");
            let mut aliases = Vec::new();
            let mut buses = Vec::new();
            for slot in 0..SLOTS {
                let kind = match slot {
                    0..=2 => GroveKind::Calc,
                    3 | 4 => GroveKind::Units,
                    5 => GroveKind::Bus,
                    _ => match splitmix64(&mut state) % 3 {
                        0 => GroveKind::Calc,
                        1 => GroveKind::Units,
                        _ => GroveKind::Bus,
                    },
                };
                let a = alias(g, s, slot);
                if let Some(hash) = hashes.get(&kind) {
                    bodies.push(BodyBinding {
                        hash: *hash,
                        alias: a.clone(),
                    });
                }
                if kind == GroveKind::Bus {
                    buses.push(a.clone());
                }
                aliases.push(a);
            }
            let mut members = vec![member(alias(g, s, 0), "sum", 2, Mark::Tail)];
            members.extend(
                buses
                    .into_iter()
                    .map(|b| member(b, "listen", 0, Mark::Head)),
            );
            links.push(Link {
                id: format!("sys_{name}"),
                order: Order::None,
                members,
            });
            systems.push(System {
                name,
                bodies: aliases,
            });
        }
        let mut members = vec![member(alias(g, 0, 1), "sum", 2, Mark::Tail)];
        members.extend((0..SYSTEMS).map(|s| member(alias(g, s, 3), "scale", 0, Mark::Head)));
        links.push(Link {
            id: format!("gal_g{g}"),
            order: Order::None,
            members,
        });
        galaxies.push(Galaxy {
            name: format!("g{g}"),
            systems,
        });
    }
    let mut members = vec![member(alias(0, 0, 2), "sum", 2, Mark::Tail)];
    members.extend((0..GALAXIES).map(|g| member(alias(g, 0, 4), "scale", 0, Mark::Head)));
    links.push(Link {
        id: "uni".to_owned(),
        order: Order::None,
        members,
    });

    Ok(Universe {
        coding: UniverseCoding {
            codex: 1,
            declarations: Vec::new(),
            bodies,
            links,
            cross_wires: Vec::new(),
            grants: BTreeMap::new(),
            lenses: vec![Lens {
                name: "function".to_owned(),
                galaxies,
            }],
        },
        regulatory: UniverseRegulatory::default(),
    })
}

#[cfg(test)]
mod tests {
    use joinn_frame::Verdict;
    use joinn_link::{Universe, parse_universe, print_universe};
    use std::collections::{BTreeMap, BTreeSet};

    use super::grow_grove;
    use crate::fns::grove::{GroveKind, KINDS};

    fn grove(seed: u64) -> Universe {
        match grow_grove(seed) {
            Ok(u) => u,
            Err(e) => panic!("{e}"),
        }
    }

    fn kind_of(u: &Universe, alias: &str) -> GroveKind {
        let Some(b) = u.coding.bodies.iter().find(|b| b.alias == alias) else {
            panic!("no body {alias}");
        };
        let Some(k) = KINDS.into_iter().find(|k| k.hash_hex() == b.hash.to_hex()) else {
            panic!("{alias} binds no grove kind");
        };
        k
    }

    #[test]
    fn seed_7_grows_the_plan_s_counts() {
        let u = grove(7);
        let mut kinds: BTreeMap<GroveKind, usize> = BTreeMap::new();
        for b in &u.coding.bodies {
            *kinds.entry(kind_of(&u, &b.alias)).or_default() += 1;
        }
        assert_eq!(kinds.get(&GroveKind::Calc), Some(&1181));
        assert_eq!(kinds.get(&GroveKind::Units), Some(&982));
        assert_eq!(kinds.get(&GroveKind::Bus), Some(&909));
        assert_eq!(u.coding.bodies.len(), 3072);
        assert_eq!(u.coding.bodies[0].alias, "b0000");
        assert_eq!(u.coding.bodies[3071].alias, "b3071");
        assert_eq!(u.coding.links.len(), 137);
        let members: usize = u.coding.links.iter().map(|l| l.members.len()).sum();
        assert_eq!(members, 1182);
        let lens = &u.coding.lenses[0];
        assert_eq!(lens.name, "function");
        assert_eq!(lens.galaxies.len(), 8);
        assert!(lens.galaxies.iter().all(|g| g.systems.len() == 16));
        assert!(
            lens.galaxies
                .iter()
                .flat_map(|g| &g.systems)
                .all(|s| s.bodies.len() == 24)
        );
        assert_eq!(lens.galaxies[7].systems[15].name, "g7s15");
    }

    #[test]
    fn the_first_six_drawn_kinds_are_calc_calc_calc_calc_units_calc() {
        let u = grove(7);
        let fixed: Vec<&str> = (0..6)
            .map(|i| kind_of(&u, &format!("b{i:04}")).name())
            .collect();
        assert_eq!(fixed, ["calc", "calc", "calc", "units", "units", "bus"]);
        let drawn: Vec<&str> = (6..12)
            .map(|i| kind_of(&u, &format!("b{i:04}")).name())
            .collect();
        assert_eq!(drawn, ["calc", "calc", "calc", "calc", "units", "calc"]);
    }

    #[test]
    fn every_port_is_in_at_most_one_link_and_tails_come_first() {
        let u = grove(7);
        let mut seen = BTreeSet::new();
        for link in &u.coding.links {
            assert_eq!(link.members[0].mark, joinn_link::Mark::Tail, "{}", link.id);
            for m in &link.members {
                let port = (m.body.clone(), m.instance.clone(), m.port);
                assert!(
                    seen.insert(port),
                    "{} repeats {}.{}@{}",
                    link.id,
                    m.body,
                    m.instance,
                    m.port
                );
            }
        }
        assert_eq!(seen.len(), 1182);
    }

    #[test]
    fn the_canonical_text_parses_back_to_the_same_coding() {
        let u = grove(7);
        let text = print_universe(&u.coding);
        let back = match parse_universe(&text) {
            Verdict::Ok(b) => b,
            Verdict::Refused(r) => panic!("{}", r.reason),
        };
        assert_eq!(print_universe(&back.coding), text);
        assert!(!text.contains('\r'));
    }
}
