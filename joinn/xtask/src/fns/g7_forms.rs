//! Gate 7 item 1: two forms, one truth.

use std::fs;

use joinn_dna::{hash, parse_contact};
use joinn_frame::{FrameRegistry, Verdict};

use super::contact::{lowered_line, surface_line, transcript_line};
use super::g7_descriptions::g7_descriptions;
use super::g7_load::g7_load;
use super::workspace_root;

/// §2.5: the lowered contact is `calculator.body`.
const LOWERED: &str = "b55fba1eff65942099f6daf84b8bc47d605be05f637d805d260d3fcfd8c3ebde";
/// §2.3's refusal for a `wires` section.
const WIRES: &str =
    "contact: wires belong to systems; a body's cells touch. acceptance is a forces section";

/// The lowered text and hash, the CLI transcript, both descriptions (bytes and
/// hash), the surface and intent sets, and the wires refusal. Every failing
/// check is printed, not only the first.
pub(crate) fn g7_forms() -> bool {
    let (contact, lowered, cells) = match g7_load() {
        Ok(loaded) => loaded,
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let corpus = match workspace_root() {
        Ok(root) => root.join("corpus"),
        Err(e) => {
            println!("{e}");
            return false;
        }
    };
    let mut failures = Vec::new();
    if let Err(e) = lowered_line(&corpus, "calculator", &lowered) {
        failures.push(e);
    }
    let hex = hash(&lowered.coding).to_hex();
    if hex != LOWERED {
        failures.push(format!("lowered hash {hex}; acceptance is {LOWERED}"));
    }
    if let Err(e) = transcript_line(&corpus, "phase7/calculator.contact", "calculator") {
        failures.push(e);
    }
    failures.extend(g7_descriptions(&corpus, &contact, &cells));
    if let Err(e) = surface_line(&contact, &lowered, &cells) {
        failures.push(e);
    }
    let path = corpus.join("phase7").join("calculator.contact");
    match fs::read_to_string(&path) {
        Ok(src) => {
            let wired = src.replacen("  forces {", "  wires { cli_a@1 -> sum@0 }\n  forces {", 1);
            if wired == src {
                failures
                    .push("calculator.contact has no forces section to put wires beside".into());
            } else {
                match parse_contact(&wired, &FrameRegistry::phase1()) {
                    Verdict::Refused(r) if r.reason == WIRES => {}
                    Verdict::Refused(r) => failures.push(format!(
                        "a wires section was refused for the wrong reason: {}",
                        r.reason
                    )),
                    Verdict::Ok(_) => failures.push("a wires section was admitted".into()),
                }
            }
        }
        Err(e) => failures.push(format!("{}: {e}", path.display())),
    }
    for f in &failures {
        println!("{f}");
    }
    failures.is_empty()
}
