//! Run one command to completion and keep what it printed.

use super::Step;
use std::path::Path;
use std::process::Command;

/// Run `program args` in `dir` with `envs` set, capturing stdout and stderr.
/// A command that cannot start is a failed step whose stderr says why.
pub(crate) fn run_step(program: &Path, args: &[&str], dir: &Path, envs: &[(&str, &Path)]) -> Step {
    #[allow(clippy::disallowed_methods)] // wall time is information (rule 4)
    let start = std::time::Instant::now();
    let mut command = Command::new(program);
    command.args(args).current_dir(dir);
    for (key, value) in envs {
        command.env(key, value);
    }
    let lines = |bytes: &[u8]| {
        String::from_utf8_lossy(bytes)
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    match command.output() {
        Ok(output) => Step {
            ok: output.status.success(),
            code: output.status.code(),
            out: lines(&output.stdout),
            err: lines(&output.stderr),
            ms: start.elapsed().as_millis(),
        },
        Err(e) => Step {
            ok: false,
            code: None,
            out: Vec::new(),
            err: vec![format!("could not start {}: {e}", program.display())],
            ms: start.elapsed().as_millis(),
        },
    }
}
