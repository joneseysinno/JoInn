//! Live engine: a fold over an explicit activation stack. Never recurses in Rust.

#![forbid(unsafe_code)]

mod activation;
mod dna;
mod ready;
mod report;
mod slot;
mod state;

pub use dna::LiveDna;
pub use report::StepReport;
pub use state::BodyState;
