//! The tower (plan 8 §2.6): the 2D Hilbert fold, segment levels, the level
//! filter, and the facts T1–T7 it must witness. All integer, no floats.

#![forbid(unsafe_code)]

mod dim;
mod direction;
mod facts;
mod level;
mod point;
mod refuse;

pub use dim::{Dim, check_dim};
pub use direction::{Direction, directions};
pub use facts::{FACT_IDS, facts, t5_filter};
pub use level::{level, level_filter};
pub use point::{Point, nodes, point};
