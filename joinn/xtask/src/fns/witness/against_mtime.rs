//! Compare a finding's mtime to every file under the given roots.
//! allow(modules): walk and format are private helpers of one mtime check

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Outcome of a witness check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Status {
    /// Finding is strictly newer than every scanned file.
    Current,
    /// A scanned file is at least as new as the finding.
    Stale { path: PathBuf, when: String },
}

/// Finding must be strictly newer (by mtime) than every file under `roots`.
pub(crate) fn against_mtime(finding: &Path, roots: &[PathBuf]) -> Result<Status, String> {
    let finding_meta = fs::metadata(finding).map_err(|e| format!("{}: {e}", finding.display()))?;
    let finding_mtime = finding_meta
        .modified()
        .map_err(|e| format!("{}: {e}", finding.display()))?;
    let mut newest: Option<(PathBuf, SystemTime)> = None;
    for root in roots {
        if !root.exists() {
            continue;
        }
        walk_files(root, &mut |path, mtime| match &newest {
            None => newest = Some((path.to_path_buf(), mtime)),
            Some((_, best)) if mtime >= *best => newest = Some((path.to_path_buf(), mtime)),
            Some(_) => {}
        })?;
    }
    let Some((path, mtime)) = newest else {
        return Ok(Status::Current);
    };
    if finding_mtime > mtime {
        return Ok(Status::Current);
    }
    Ok(Status::Stale {
        path,
        when: format_system_time(mtime),
    })
}

fn walk_files(dir: &Path, f: &mut impl FnMut(&Path, SystemTime)) -> Result<(), String> {
    let rd = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut ents: Vec<_> = rd
        .map(|e| e.map_err(|err| format!("{}: {err}", dir.display())))
        .collect::<Result<Vec<_>, _>>()?;
    ents.sort_by_key(|e| e.file_name());
    for ent in ents {
        let path = ent.path();
        let meta = fs::metadata(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if meta.is_dir() {
            walk_files(&path, f)?;
            continue;
        }
        if meta.is_file() {
            let mtime = meta
                .modified()
                .map_err(|e| format!("{}: {e}", path.display()))?;
            f(&path, mtime);
        }
    }
    Ok(())
}

fn format_system_time(t: SystemTime) -> String {
    match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => format!("unix {}", d.as_secs()),
        Err(_) => "unix before-epoch".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Status, against_mtime};
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn aged_finding_below_temp_tree_is_stale() {
        let root =
            std::env::temp_dir().join(format!("joinn-witness-{}-{}", std::process::id(), "aged"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("crates")).unwrap_or_else(|e| panic!("mkdir crates: {e}"));
        let finding = root.join("finding.md");
        fs::write(&finding, "old").unwrap_or_else(|e| panic!("write finding: {e}"));
        thread::sleep(Duration::from_millis(1100));
        let newer = root.join("crates").join("newer.rs");
        fs::write(&newer, "new").unwrap_or_else(|e| panic!("write newer: {e}"));
        let status = against_mtime(&finding, &[root.join("crates")])
            .unwrap_or_else(|e| panic!("against_mtime check: {e}"));
        match status {
            Status::Stale { path, when } => {
                assert_eq!(path, newer);
                assert!(when.contains("unix"), "{when}");
            }
            Status::Current => panic!("aged finding must be stale"),
        }
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn finding_newer_than_temp_tree_is_current() {
        let root =
            std::env::temp_dir().join(format!("joinn-witness-{}-{}", std::process::id(), "fresh"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("crates")).unwrap_or_else(|e| panic!("mkdir crates: {e}"));
        let older = root.join("crates").join("older.rs");
        fs::write(&older, "old").unwrap_or_else(|e| panic!("write older: {e}"));
        thread::sleep(Duration::from_millis(1100));
        let finding = root.join("finding.md");
        fs::write(&finding, "new").unwrap_or_else(|e| panic!("write finding: {e}"));
        let status = against_mtime(&finding, &[root.join("crates")])
            .unwrap_or_else(|e| panic!("against_mtime check: {e}"));
        assert_eq!(status, Status::Current);
        let _ = fs::remove_dir_all(&root);
    }
}
