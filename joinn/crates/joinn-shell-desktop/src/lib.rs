//! L0: the desktop shell. A window that renders on demand and a host that picks.

#![forbid(unsafe_code)]

mod load;
mod session;
mod window;

pub use window::run;
