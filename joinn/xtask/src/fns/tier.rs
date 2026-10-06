//! Checks by tier (plan 7.3 §2.11 d): `cargo xtask check` on every commit,
//! `cargo xtask stop-check [--fresh]` once per phase. Each runs commands and
//! reports what they printed; neither chooses a count.

mod check;
mod ci_steps;
mod run_step;
mod stop_check;
mod test_counts;

pub(crate) use check::check;
pub(crate) use stop_check::stop_check;

/// One command's run: its exit, its stdout and stderr lines, its wall time.
pub(crate) struct Step {
    pub(crate) ok: bool,
    pub(crate) code: Option<i32>,
    pub(crate) out: Vec<String>,
    pub(crate) err: Vec<String>,
    pub(crate) ms: u128,
}

impl Step {
    /// The last non-empty stdout line, else the last non-empty stderr line.
    pub(crate) fn last_line(&self) -> &str {
        self.out
            .iter()
            .rev()
            .find(|l| !l.trim().is_empty())
            .or_else(|| self.err.iter().rev().find(|l| !l.trim().is_empty()))
            .map_or("", |l| l.trim_end())
    }
}

/// A step of the CI workflow: its name, its `run:` line, and whether it runs
/// on Linux only.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct CiStep {
    pub(crate) name: String,
    pub(crate) run: String,
    pub(crate) linux_only: bool,
}
