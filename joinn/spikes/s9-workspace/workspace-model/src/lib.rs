//! Cells, nesting, the time log, and formulas for the workspace sketch.
//! Nothing in this crate draws a screen.

#![forbid(unsafe_code)]

mod fraction;
mod log;
mod sheet;

pub use fraction::Fraction;
pub use log::{Document, Entry, Kind, Op, StoreError};
pub use sheet::{
    Arrangement, Cell, CellId, Inside, Link, NewCell, Placed, Refusal, Snapshot, StoredCell,
    Worksheet,
};
