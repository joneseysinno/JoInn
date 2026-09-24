//! Checkpoint currency: a finding must be newer than crates, xtask, and corpus.

mod against_git;
mod against_mtime;
mod resolve_path;
mod run;

pub(crate) use run::witness;
