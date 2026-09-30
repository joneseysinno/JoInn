//! §2.3's four spellings of the contact calculator, and the grant-order fifth.

use joinn_dna::hash;
use joinn_dna::parse_contact;
use joinn_frame::{FrameRegistry, Verdict};

/// §2.3: the corpus contact's hash.
const CONTACT: &str = "868e79b293c8d35625f514274eb0bf04d228898ed994a795b23b942a530b6209";
const CLI: &str = "c4a0a132c4b63027083b81e14b1c1055858a00cc03655661b2434a2e2f4ed17e";
const SUM: &str = "6b3271631abf49a3afdd852cea78a71ab6aa99598eb1405d1db051169e624c39";

/// `src` is the corpus text. Its three respellings (members reversed with
/// sections reordered and comments, one line, a regulatory-only change) hash
/// to `868e79b2…`; the grant-order swap does not. Each break is one failure.
pub(crate) fn g7_spellings(src: &str) -> Vec<String> {
    let frames = FrameRegistry::phase1();
    let reordered = format!(
        "# members reversed, sections reordered\ncontact {{\n  lineage none\n  budget {{ steps 100000 }}   # the budget first\n  forces {{\n    combine ℤ 1 cell:{SUM} as sum from cli_b@1 cli_a@1\n  }}\n  grants {{ stdin: cli_a, cli_b }}\n  genome {{ cell:{CLI} as cli_b, cli_a }}\n  codex 1\n}}\n---\nregulatory {{\n  prompts {{ cli_a \"a: \"  cli_b \"b: \" }}\n  present {{ sum \"{{0}} + {{1}} = {{2}}\" }}\n}}\n"
    );
    let one_line = format!(
        "contact {{ codex 1 genome {{ cell:{CLI} as cli_a, cli_b }} grants {{ stdin cli_a cli_b }} forces {{ combine ℤ 1 cell:{SUM} as sum from cli_a@1, cli_b@1 }} budget {{ steps 100000 }} lineage none }}\n"
    );
    let regulatory = src.replace(r#"labels { sum "Sum" }"#, r#"labels { sum "Total" }"#);
    let swapped = src.replace("stdin: cli_a, cli_b", "stdin: cli_b, cli_a");
    let mut failures = Vec::new();
    if regulatory == src || swapped == src {
        failures
            .push("spellings: the corpus text lacks the label or grant line §2.3 edits".to_owned());
        return failures;
    }
    let spellings = [
        ("the corpus source", src),
        ("members reversed, sections reordered", reordered.as_str()),
        ("one line", one_line.as_str()),
        ("regulatory only", regulatory.as_str()),
    ];
    for (name, text) in spellings {
        match parse_contact(text, &frames) {
            Verdict::Ok(c) => {
                let hex = hash(&c.coding).to_hex();
                if hex != CONTACT {
                    failures.push(format!("spelling {name}: hash {hex}, not {CONTACT}"));
                }
            }
            Verdict::Refused(r) => failures.push(format!("spelling {name}: refused: {}", r.reason)),
        }
    }
    match parse_contact(&swapped, &frames) {
        Verdict::Ok(c) if hash(&c.coding).to_hex() == CONTACT => failures
            .push("spelling grant order swapped: same hash; grant order is hashed (G4)".into()),
        Verdict::Ok(_) => {}
        Verdict::Refused(r) => failures.push(format!(
            "spelling grant order swapped: refused: {}",
            r.reason
        )),
    }
    failures
}
