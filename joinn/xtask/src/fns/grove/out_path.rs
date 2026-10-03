//! Where `--out` may write: anywhere but corpus/ and docs/ (rule 73).

use std::path::{Component, Path, PathBuf};

/// `path` made absolute against `cwd` and normalised lexically. A path under
/// the workspace's `corpus/` or the repository's `docs/` is refused.
pub(crate) fn out_path(path: &str, cwd: &Path, workspace: &Path) -> Result<PathBuf, String> {
    let mut abs = PathBuf::new();
    for part in cwd.join(path).components() {
        match part {
            Component::ParentDir => {
                abs.pop();
            }
            Component::CurDir => {}
            other => abs.push(other.as_os_str()),
        }
    }
    let mut banned = vec![workspace.join("corpus"), workspace.join("docs")];
    if let Some(repo) = workspace.parent() {
        banned.push(repo.join("docs"));
    }
    if banned.iter().any(|b| abs.starts_with(b)) {
        return Err(format!(
            "grove: --out {path} is under corpus/ or docs/; acceptance is a path outside corpus/ and docs/ (rule 73)"
        ));
    }
    Ok(abs)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::out_path;

    fn workspace() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
    }

    #[test]
    fn out_under_corpus_or_docs_is_refused() {
        let ws = match workspace().canonicalize() {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
        for p in [
            "corpus/x.universe",
            "./corpus/phase5/x.universe",
            "../docs/x.universe",
            "target/../corpus/x",
        ] {
            match out_path(p, &ws, &ws) {
                Err(e) => assert_eq!(
                    e,
                    format!(
                        "grove: --out {p} is under corpus/ or docs/; acceptance is a path outside corpus/ and docs/ (rule 73)"
                    )
                ),
                Ok(abs) => panic!("{p} → {} must be refused", abs.display()),
            }
        }
    }

    #[test]
    fn out_under_target_is_allowed() {
        let ws = match workspace().canonicalize() {
            Ok(p) => p,
            Err(e) => panic!("{e}"),
        };
        match out_path("target/grove.universe", &ws, &ws) {
            Ok(abs) => assert_eq!(abs, ws.join("target").join("grove.universe")),
            Err(e) => panic!("{e}"),
        }
    }
}
