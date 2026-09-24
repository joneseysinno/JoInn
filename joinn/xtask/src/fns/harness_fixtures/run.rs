//! Run every harness fixture before any gate table.

use std::sync::OnceLock;

use super::lock_round_trip::check_lock_round_trip;
use super::opposing::check_opposing_fixtures;

static HARNESS: OnceLock<Result<(), String>> = OnceLock::new();

/// Opposition fixtures and lock round-trip. Stops the run if any come out wrong.
pub(crate) fn harness_fixtures() -> Result<(), String> {
    HARNESS
        .get_or_init(|| {
            check_opposing_fixtures()?;
            check_lock_round_trip()?;
            println!("harness fixtures: ok");
            Ok(())
        })
        .as_ref()
        .map(|_| ())
        .map_err(String::clone)
}
