//! Lens exclusivity: one owning system per body per lens.

use joinn_frame::Verdict;
use joinn_link::{check_lenses, parse_universe};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

fn corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

#[test]
fn two_systems_refuses_naming_the_body_and_both_systems() {
    let src = match fs::read_to_string(
        corpus()
            .join("phase5")
            .join("controls")
            .join("two_systems.universe"),
    ) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_lenses(&u) {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("units"), "{}", r.reason);
            assert!(r.reason.contains("calculation"), "{}", r.reason);
            assert!(r.reason.contains("measurement"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("units in two systems of one lens must refuse"),
    }
}

#[test]
fn units_sits_in_two_systems_across_two_lenses() {
    let src = match fs::read_to_string(corpus().join("phase5").join("universe.universe")) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    match check_lenses(&u) {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    let mut systems = BTreeSet::new();
    for lens in &u.coding.lenses {
        let mut found = false;
        for galaxy in &lens.galaxies {
            for system in &galaxy.systems {
                if system.bodies.iter().any(|a| a == "units") {
                    assert!(!found, "units twice inside lens {}", lens.name);
                    found = true;
                    systems.insert(system.name.clone());
                }
            }
        }
        assert!(found, "units missing from lens {}", lens.name);
    }
    assert_eq!(
        systems,
        BTreeSet::from(["local".to_string(), "measurement".to_string()])
    );
}
