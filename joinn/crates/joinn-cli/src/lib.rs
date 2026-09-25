//! CLI host as a library for gate checks. Binary is `joinn`.

mod cli_host;
mod load;
mod present;
mod session;

pub use session::present_universe;
pub use session::run_named as session_run;
