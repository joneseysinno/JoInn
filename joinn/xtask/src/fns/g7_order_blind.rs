//! Gate 7 item 2: combine is order-blind, and it fits what it reaches.

use std::fs;

use joinn_frame::FrameRegistry;

use super::contact::pairings;
use super::forces;
use super::g7_load::g7_load;
use super::g7_spellings::g7_spellings;
use super::workspace_root;

/// `pairings` names every member→port order and one sum.
const PAIRINGS: &str = "pairings: 2, sum@2 = 5 in each";

/// `cargo xtask forces` (the register, `order_blind` on its four plants, and the
/// unopposed plant), every pairing summing to 5, and §2.3's four spellings.
pub(crate) fn g7_order_blind() -> bool {
    let mut failures = Vec::new();
    if let Err(e) = forces() {
        failures.push(e);
    }
    let (contact, _, cells) = match g7_load() {
        Ok(loaded) => loaded,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    match pairings(&contact, &cells, &FrameRegistry::phase1()) {
        Ok(line) if line == PAIRINGS => {}
        Ok(line) => failures.push(format!("{line}; acceptance is `{PAIRINGS}`")),
        Err(e) => failures.push(e),
    }
    let src = workspace_root().and_then(|root| {
        let path = root
            .join("corpus")
            .join("phase7")
            .join("calculator.contact");
        fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))
    });
    match src {
        Ok(src) => failures.extend(g7_spellings(&src.replace("\r\n", "\n"))),
        Err(e) => failures.push(e),
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
