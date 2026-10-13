//! The text of a lock file for the scores each gate returned.

/// The header line, then one `<phase>: <score>` line per outcome.
pub(crate) fn lock_text(outcomes: &[(&str, bool, u32, u32, bool)]) -> String {
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
    body
}
