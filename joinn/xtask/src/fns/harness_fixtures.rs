//! Harness fixtures: opposition and lock round-trip, before any gate table.

mod check_ok;
mod honest_crossing;
mod ignores_subject;
mod lock_round_trip;
mod opposing;
mod oversensitive;
mod run;

pub(crate) use run::harness_fixtures;
