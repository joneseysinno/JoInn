//! Incidence combinatorics over an incidence table. This crate names no DNA type.

#![forbid(unsafe_code)]

mod block;
mod chain;
mod complex;

pub use block::BlockId;
pub use chain::Chain;
pub use complex::{Complex, Homology};
