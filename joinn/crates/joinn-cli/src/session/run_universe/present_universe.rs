//! Bind a store and present a universe from scripted lines.

use joinn_frame::Verdict;
use joinn_link::{BodyStore, Universe, bind};
use std::io;

use super::present_bound::present_bound;

/// Drive a universe from scripted lines. Used by the CLI and gate 5.2.
pub fn present_universe(
    universe: &Universe,
    store: &BodyStore,
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<String, String> {
    let bound = match bind(universe, store) {
        Verdict::Ok(b) => b,
        Verdict::Refused(r) => return Err(r.reason),
    };
    present_bound(universe, &bound, lines)
}
