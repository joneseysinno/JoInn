//! A checkpoint is unaccepted until AJ's sign-off line is present (§2.9).

use std::fs;
use std::path::Path;

/// Refuse unless the finding contains a line matching `^Accepted: AJ, `.
pub(crate) fn requires_acceptance(path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if text.lines().any(|line| line.starts_with("Accepted: AJ, ")) {
        return Ok(());
    }
    Err(format!("unaccepted: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::requires_acceptance;
    use std::fs;

    #[test]
    fn missing_accepted_line_is_unaccepted() {
        let path = std::env::temp_dir().join(format!(
            "joinn-witness-unaccepted-{}",
            std::process::id()
        ));
        fs::write(&path, "# Checkpoint\n\nEvidence only.\n").expect("write");
        let err = requires_acceptance(&path).expect_err("must refuse");
        assert!(
            err.starts_with("unaccepted:"),
            "got {err}"
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn accepted_line_admits() {
        let path = std::env::temp_dir().join(format!(
            "joinn-witness-accepted-{}",
            std::process::id()
        ));
        fs::write(&path, "# Checkpoint\n\nAccepted: AJ, 24 Sep 2026\n").expect("write");
        requires_acceptance(&path).expect("must admit");
        let _ = fs::remove_file(&path);
    }
}
