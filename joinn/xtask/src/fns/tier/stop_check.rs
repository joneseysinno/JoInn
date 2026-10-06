//! `cargo xtask stop-check [--fresh]`: the phase-stop tier (Run 7.3–9 §3).

use super::ci_steps::ci_steps;
use super::run_step::run_step;
use super::test_counts::test_counts;
use crate::fns::{say, workspace_root};
use std::path::Path;

/// Run every CI step in workflow order (Linux-only steps only on Linux),
/// printing each step's last line and ms as it ends, then every line that says
/// "fail", every failing test, and the total. `--fresh` first clones the
/// committed tree under `target/fresh` and runs there with its own target dir.
pub(crate) fn stop_check(args: Vec<String>) -> Result<(), String> {
    let fresh = match args.as_slice() {
        [] => false,
        [flag] if flag == "--fresh" => true,
        _ => return Err("usage: cargo xtask stop-check [--fresh]".into()),
    };
    #[allow(clippy::disallowed_methods)] // wall time is information (rule 4)
    let start = std::time::Instant::now();
    let root = workspace_root()?;
    let yaml = std::fs::read_to_string(root.join("../.github/workflows/ci.yml"))
        .map_err(|e| format!("stop-check: ci.yml: {e}"))?;
    let steps = ci_steps(&yaml);
    let git = Path::new("git");

    let fresh_dir = root.join("target").join("fresh");
    let (dir, target) = if fresh {
        if fresh_dir.exists() {
            std::fs::remove_dir_all(&fresh_dir)
                .map_err(|e| format!("stop-check: clear {}: {e}", fresh_dir.display()))?;
        }
        let repo = root.join("..");
        let repo = repo.to_string_lossy();
        let dest = fresh_dir.to_string_lossy();
        let clone = run_step(
            git,
            &["clone", "-q", "--no-hardlinks", &repo, &dest],
            &root,
            &[],
        );
        if !clone.ok {
            return Err(format!("stop-check: clone: {}", clone.last_line()));
        }
        let dir = fresh_dir.join("joinn");
        let target = dir.join("target");
        (dir, Some(target))
    } else {
        (root.clone(), None)
    };
    let head = run_step(git, &["rev-parse", "--short", "HEAD"], &dir, &[]);
    say(&format!(
        "stop-check: {} at {}{}",
        dir.display(),
        head.last_line(),
        if fresh { " (fresh clone)" } else { "" }
    ));
    let envs: Vec<(&str, &Path)> = target
        .as_deref()
        .map(|t| vec![("CARGO_TARGET_DIR", t)])
        .unwrap_or_default();
    let (shell, flag) = if cfg!(windows) {
        (Path::new("cmd"), "/C")
    } else {
        (Path::new("sh"), "-c")
    };

    let mut fail_lines = Vec::new();
    let mut failing_tests = Vec::new();
    let mut failed_steps = Vec::new();
    for step in &steps {
        if step.linux_only && !cfg!(target_os = "linux") {
            say(&format!(
                "{}: {} → skipped (Linux only)",
                step.name, step.run
            ));
            continue;
        }
        let ran = run_step(shell, &[flag, &step.run], &dir, &envs);
        let verdict = match (ran.ok, ran.code) {
            (true, _) => String::new(),
            (false, Some(code)) => format!("; failed, exit {code}"),
            (false, None) => "; failed, exit without a code".to_owned(),
        };
        let (passed, failed, names) = test_counts(&ran.out);
        let ran_tests = ran.out.iter().any(|l| l.starts_with("test result:"));
        let said = match (ran_tests, ran.last_line(), ran.code) {
            (true, _, _) => format!("{passed} passed, {failed} failed"),
            (false, "", Some(code)) => format!("exit {code}"),
            (false, last, _) => last.to_owned(),
        };
        say(&format!(
            "{}: {} → {said} ({} ms, information){verdict}",
            step.name, step.run, ran.ms
        ));
        for line in ran.out.iter().chain(&ran.err) {
            let lower = line.to_lowercase();
            if lower.contains("fail")
                && !line.starts_with("test result:")
                && !line.trim_end().ends_with(" ... ok")
            {
                fail_lines.push(format!("{}: {}", step.name, line.trim_end()));
            }
        }
        failing_tests.extend(names.into_iter().map(|n| format!("{}: {n}", step.name)));
        if !ran.ok {
            failed_steps.push(step.name.clone());
        }
    }

    for line in &fail_lines {
        say(&format!("says fail: {line}"));
    }
    for name in &failing_tests {
        say(&format!("failing test: {name}"));
    }
    let ms = start.elapsed().as_millis();
    if failed_steps.is_empty() {
        say(&format!("stop-check: ok ({ms} ms, information)"));
        Ok(())
    } else {
        Err(format!(
            "stop-check: failed: {} ({ms} ms, information)",
            failed_steps.join(", ")
        ))
    }
}
