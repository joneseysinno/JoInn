//! Compare commit times when a git repository is present (§2.12).
//! allow(modules): git helpers share one against_git entry point

use super::against_mtime::Status;
use std::path::{Path, PathBuf};
use std::process::Command;

/// `Ok(None)` means no repository — caller falls back to mtimes.
pub(crate) fn against_git(finding: &Path, workspace: &Path) -> Result<Option<Status>, String> {
    let Some(repo) = repo_root(workspace) else {
        return Ok(None);
    };
    let finding_rel = path_relative_to(&repo, finding)?;
    let finding_ct = match git_commit_unix(&repo, &[&finding_rel])? {
        Some(t) => t,
        None => {
            return Err(format!(
                "witness: {} is not in git; acceptance is a committed finding",
                finding.display()
            ));
        }
    };
    let trees = ["joinn/crates", "joinn/xtask", "joinn/corpus"];
    let code_ct = match git_commit_unix(&repo, &trees)? {
        Some(t) => t,
        None => return Ok(Some(Status::Current)),
    };
    if finding_ct > code_ct {
        return Ok(Some(Status::Current));
    }
    let (path, when) = git_newest_touch(&repo, &trees, code_ct)?;
    Ok(Some(Status::Stale { path, when }))
}

fn repo_root(start: &Path) -> Option<PathBuf> {
    let mut cur = start.to_path_buf();
    loop {
        if cur.join(".git").exists() {
            return Some(cur);
        }
        if !cur.pop() {
            return None;
        }
    }
}

fn path_relative_to(repo: &Path, path: &Path) -> Result<String, String> {
    let abs = fs_canonicalize(path)?;
    let root = fs_canonicalize(repo)?;
    let rel = abs.strip_prefix(&root).map_err(|_| {
        format!(
            "witness: {} is outside the repository {}",
            path.display(),
            repo.display()
        )
    })?;
    Ok(rel.to_string_lossy().replace('\\', "/"))
}

fn fs_canonicalize(path: &Path) -> Result<PathBuf, String> {
    std::fs::canonicalize(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn git_commit_unix(repo: &Path, paths: &[&str]) -> Result<Option<u64>, String> {
    let mut args = vec![
        "-C".to_string(),
        repo.display().to_string(),
        "log".into(),
        "-1".into(),
        "--format=%ct".into(),
        "--".into(),
    ];
    for p in paths {
        args.push((*p).into());
    }
    let out = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if text.is_empty() {
        return Ok(None);
    }
    let secs: u64 = text
        .parse()
        .map_err(|_| format!("git: bad commit time {text}"))?;
    Ok(Some(secs))
}

fn git_newest_touch(
    repo: &Path,
    paths: &[&str],
    code_ct: u64,
) -> Result<(PathBuf, String), String> {
    let mut args = vec![
        "-C".to_string(),
        repo.display().to_string(),
        "log".into(),
        "-1".into(),
        "--name-only".into(),
        "--pretty=format:".into(),
        "--".into(),
    ];
    for p in paths {
        args.push((*p).into());
    }
    let out = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    let text = String::from_utf8_lossy(&out.stdout);
    let rel = text
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or(paths[0]);
    let when = git_commit_iso(repo, paths)?.unwrap_or_else(|| format!("unix {code_ct}"));
    Ok((repo.join(rel), when))
}

fn git_commit_iso(repo: &Path, paths: &[&str]) -> Result<Option<String>, String> {
    let mut args = vec![
        "-C".to_string(),
        repo.display().to_string(),
        "log".into(),
        "-1".into(),
        "--format=%ci".into(),
        "--".into(),
    ];
    for p in paths {
        args.push((*p).into());
    }
    let out = Command::new("git")
        .args(&args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !out.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if text.is_empty() {
        Ok(None)
    } else {
        Ok(Some(text))
    }
}
