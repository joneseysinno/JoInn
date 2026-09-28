//! `cargo xtask regrow`: tables built by deltas equal tables regrown from DNA and
//! live state (V122), and GPU state dropped and regrown draws the same bytes (VH2).

mod cleared_elsewhere;
mod drive;
mod event;
mod run;

pub(crate) use run::regrow;

/// One scripted input: the instance whose port 0 it types into, and the text.
pub(crate) type Input = (&'static str, &'static str);
