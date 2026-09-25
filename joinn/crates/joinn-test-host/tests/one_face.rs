//! One face per coding hash in a body set.

use joinn_dna::{hash, parse_body};
use joinn_frame::{FrameRegistry, Verdict};
use joinn_test_host::load_body_set;
use std::fs;
use std::path::Path;

fn corpus() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
}

#[test]
fn duplicate_face_refuses_naming_both_paths() {
    let root = corpus();
    let frames = FrameRegistry::phase1();
    let calc = root.join("phase2").join("calculator.body");
    let variant = root
        .join("phase2")
        .join("variants")
        .join("calculator_c.body");
    let src_a = fs::read_to_string(&calc).unwrap_or_else(|e| panic!("read calculator.body: {e}"));
    let src_c =
        fs::read_to_string(&variant).unwrap_or_else(|e| panic!("read calculator_c.body: {e}"));
    let body_a = match parse_body(&src_a, &frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let body_c = match parse_body(&src_c, &frames) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    assert_eq!(hash(&body_a.coding), hash(&body_c.coding));
    assert_ne!(body_a.regulatory, body_c.regulatory);

    let err = match load_body_set(vec![(calc.clone(), body_a), (variant.clone(), body_c)]) {
        Err(e) => e,
        Ok(v) => panic!("load_body_set must refuse duplicate face, got {v:?}"),
    };
    let a = calc.display().to_string();
    let c = variant.display().to_string();
    assert!(
        err.contains(&a) && err.contains(&c),
        "refusal must name both paths, got: {err}"
    );
}
