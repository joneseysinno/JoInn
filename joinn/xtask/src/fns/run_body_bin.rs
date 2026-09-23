//! Run `joinn run <stem>` with given stdin bytes.

use super::workspace_root;
use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

pub(crate) fn run_body_bin(stem: &str, stdin: &[u8]) -> Result<String, String> {
    let root = workspace_root()?;
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut child = Command::new(cargo)
        .args([
            "run",
            "-p",
            "joinn-cli",
            "--offline",
            "--quiet",
            "--",
            "run",
            stem,
        ])
        .current_dir(&root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    if let Some(mut stdin_h) = child.stdin.take() {
        stdin_h.write_all(stdin).map_err(|e| e.to_string())?;
    }
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "joinn run {stem} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}
