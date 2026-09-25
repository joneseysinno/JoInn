//! Headless host. Captures descriptions into memory. No IO.

#![forbid(unsafe_code)]

mod capture;
mod host;
mod host_intent_set;
mod load_body_set;
mod raw_event;
mod run;
mod run_universe;
mod value_of;

pub use capture::Capture;
pub use host::TestHost;
pub use host_intent_set::host_intent_set;
pub use load_body_set::load_body_set;
pub use raw_event::RawEvent;
pub use run::run;
pub use run_universe::run_universe;
