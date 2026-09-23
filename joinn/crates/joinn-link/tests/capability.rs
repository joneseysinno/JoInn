//! Grant then revoke across an ordered hyperedge, in that order.

use joinn_frame::Verdict;
use joinn_link::{LinkRuntime, check_capability, grant, parse_universe, revoke};
use std::fs;
use std::path::PathBuf;

#[test]
fn grant_succeeds_then_revoke_refuses_naming_the_capability() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("corpus")
        .join("phase5")
        .join("universe.universe");
    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => panic!("{}: {e}", path.display()),
    };
    let u = match parse_universe(&src) {
        Verdict::Ok(u) => u,
        Verdict::Refused(r) => panic!("{}", r.reason),
    };
    let mut rt = LinkRuntime::default();
    match grant(&mut rt, &u, "g0", "units") {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match check_capability(&rt, "g0", "units") {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("pre-revoke must succeed: {}", r.reason),
    }
    match revoke(&mut rt, "g0", "units") {
        Verdict::Ok(()) => {}
        Verdict::Refused(r) => panic!("{}", r.reason),
    }
    match check_capability(&rt, "g0", "units") {
        Verdict::Refused(r) => {
            assert!(r.reason.contains("g0"), "{}", r.reason);
            assert!(r.reason.contains("acceptance"), "{}", r.reason);
        }
        Verdict::Ok(()) => panic!("post-revoke must refuse naming the capability"),
    }
}
