//! Compare a run's scores with the committed lock before writing it.

use std::fs;
use std::path::Path;

use super::parse_lock_scores::parse_lock_scores;
use super::write_lock::write_lock;

/// Every phase the committed lock records must come back exactly as recorded;
/// a phase the lock does not hold yet may be added. Any other difference is
/// printed and the file is left untouched. Returns whether the lock was
/// written (false when its bytes would not change).
pub(crate) fn settle_lock(
    path: &Path,
    outcomes: &[(&str, bool, u32, u32, bool)],
) -> Result<bool, String> {
    let rows = match fs::read_to_string(path) {
        Ok(text) => parse_lock_scores(&text)?,
        Err(_) => Vec::new(),
    };
    let score = |n: u32, total: u32, legacy: bool| {
        format!("{n}/{total}{}", if legacy { " legacy" } else { "" })
    };
    let mut differences = Vec::new();
    for row in &rows {
        match outcomes.iter().find(|(phase, ..)| *phase == row.phase) {
            None => differences.push(format!(
                "{}: the lock records {}; this run has no such phase",
                row.phase,
                score(row.n, row.total, row.legacy)
            )),
            Some((_, _, n, total, legacy))
                if (*n, *total, *legacy) != (row.n, row.total, row.legacy) =>
            {
                differences.push(format!(
                    "{}: the lock records {}; this run returned {}",
                    row.phase,
                    score(row.n, row.total, row.legacy),
                    score(*n, *total, *legacy)
                ));
            }
            Some(_) => {}
        }
    }
    if !differences.is_empty() {
        return Err(format!(
            "gates.lock left unchanged; acceptance is every recorded phase as recorded\n{}",
            differences.join("\n")
        ));
    }
    write_lock(path, outcomes)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    use super::settle_lock;
    use crate::fns::workspace_root;

    fn lock_path(name: &str) -> PathBuf {
        let dir = workspace_root()
            .unwrap_or_else(|e| panic!("{e}"))
            .join("target")
            .join("settle_lock");
        fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("{e}"));
        let path = dir.join(name);
        let _ = fs::remove_file(&path);
        path
    }

    fn age(path: &PathBuf) -> SystemTime {
        let old = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000);
        let file = fs::File::options()
            .write(true)
            .open(path)
            .unwrap_or_else(|e| panic!("{e}"));
        file.set_modified(old).unwrap_or_else(|e| panic!("{e}"));
        old
    }

    fn modified(path: &PathBuf) -> SystemTime {
        fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or_else(|e| panic!("{e}"))
    }

    #[test]
    fn an_unchanged_lock_keeps_its_bytes_and_timestamp() {
        let path = lock_path("unchanged.lock");
        let (n, total) = (3, 3);
        let outcomes = [
            ("phase 0", true, 1, 1, false),
            ("phase x", true, n, total, false),
        ];
        assert_eq!(settle_lock(&path, &outcomes), Ok(true));
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{e}"));
        let old = age(&path);
        assert_eq!(settle_lock(&path, &outcomes), Ok(false));
        assert_eq!(fs::read(&path).unwrap_or_else(|e| panic!("{e}")), bytes);
        assert_eq!(modified(&path), old);
    }

    #[test]
    fn a_lower_outcome_is_printed_and_the_lock_is_untouched() {
        let path = lock_path("lower.lock");
        let (n, total) = (3, 3);
        let recorded = [
            ("phase 0", true, 1, 1, false),
            ("phase x", true, n, total, false),
        ];
        assert_eq!(settle_lock(&path, &recorded), Ok(true));
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{e}"));
        let old = age(&path);
        let lower = [
            ("phase 0", true, 1, 1, false),
            ("phase x", false, n - 1, total, false),
        ];
        match settle_lock(&path, &lower) {
            Err(msg) => assert!(
                msg.ends_with("phase x: the lock records 3/3; this run returned 2/3"),
                "{msg}"
            ),
            Ok(wrote) => panic!("a lower outcome was settled (wrote {wrote})"),
        }
        assert_eq!(fs::read(&path).unwrap_or_else(|e| panic!("{e}")), bytes);
        assert_eq!(modified(&path), old);
    }

    #[test]
    fn a_new_phase_is_added() {
        let path = lock_path("added.lock");
        let (n, total) = (3, 3);
        let recorded = [("phase 0", true, 1, 1, false)];
        assert_eq!(settle_lock(&path, &recorded), Ok(true));
        let added = [
            ("phase 0", true, 1, 1, false),
            ("phase y", true, n, total, false),
        ];
        assert_eq!(settle_lock(&path, &added), Ok(true));
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{e}"));
        assert!(text.ends_with("phase 0: pass\nphase y: 3/3\n"), "{text}");
    }
}
