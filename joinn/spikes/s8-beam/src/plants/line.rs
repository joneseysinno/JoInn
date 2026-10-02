use crate::verdict::Verdict;

/// A plant's printed line, and whether it was refused (ok).
pub fn line<T>(name: &str, verdict: Verdict<T>) -> (String, bool) {
    match verdict {
        Verdict::Refused(refusal) => (
            format!("plant {name}: refused (ok): {}", refusal.reason()),
            true,
        ),
        Verdict::Admitted(_) => (
            format!("plant {name}: admitted (not ok): no check refused it"),
            false,
        ),
    }
}
