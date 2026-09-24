//! Run every harness fixture before any gate table.

use std::sync::OnceLock;

use super::distinct_opposition::check_distinct_opposition;
use super::lock_round_trip::check_lock_round_trip;
use super::opposing::check_opposing_fixtures;

static HARNESS: OnceLock<Result<(), String>> = OnceLock::new();

/// Distinct opposition, then opposition fixtures and lock round-trip.
pub(crate) fn harness_fixtures() -> Result<(), String> {
    HARNESS
        .get_or_init(|| {
            check_distinct_opposition()?;
            check_opposing_fixtures()?;
            check_lock_round_trip()?;
            println!("harness fixtures: ok");
            Ok(())
        })
        .as_ref()
        .map(|_| ())
        .map_err(String::clone)
}
