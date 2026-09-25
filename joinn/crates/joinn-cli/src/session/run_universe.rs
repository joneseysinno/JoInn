//! Run the phase 5 universe: every line enters the universe; no side BodyState.

mod append_reports;
mod present_bound;
mod present_universe;
mod run_from_corpus;

pub use present_universe::present_universe;
pub(in crate::session) use run_from_corpus::run_universe;
