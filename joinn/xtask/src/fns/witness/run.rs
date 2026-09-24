//! `cargo xtask witness <path>` — print current or refuse as stale / unaccepted.

use super::against_git::against_git;
use super::against_mtime::{Status, against_mtime};
use super::requires_acceptance::requires_acceptance;
use super::resolve_path::resolve_path;
use super::super::workspace_root;

/// Read a finding and refuse unless accepted and newer than crates, xtask, and corpus.
pub(crate) fn witness(arg: String) -> Result<(), String> {
    let finding = resolve_path(&arg)?;
    requires_acceptance(&finding)?;
    let ws = workspace_root()?;
    let status = match against_git(&finding, &ws)? {
        Some(s) => s,
        None => against_mtime(
            &finding,
            &[
                ws.join("crates"),
                ws.join("xtask"),
                ws.join("corpus"),
            ],
        )?,
    };
    match status {
        Status::Current => {
            println!("current");
            Ok(())
        }
        Status::Stale { path, when } => Err(format!("stale: {} {when}", path.display())),
    }
}
