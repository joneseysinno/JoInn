//! Write gates.lock from the scores each gate returned.

use std::fs;

use super::workspace_root;

pub(crate) fn write_lock(outcomes: &[(&str, bool, u32, u32, bool)]) -> Result<(), String> {
    let lock = workspace_root()?.join("gates.lock");
    let mut body = String::from(
        "# JoInn gates.lock — recorded passed gates. Never edit to make a check pass.\n",
    );
    for (name, ok, n, total, legacy) in outcomes {
        if *name == "phase 0" {
            body.push_str("phase 0: ");
            body.push_str(if *ok { "pass" } else { "fail" });
            body.push('\n');
        } else if *legacy {
            body.push_str(&format!("{name}: {n}/{total} legacy\n"));
        } else {
            body.push_str(&format!("{name}: {n}/{total}\n"));
        }
    }
    fs::write(lock, body).map_err(|e| e.to_string())
}
