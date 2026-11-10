//! Cells, nesting, the time log, and formulas for the workspace sketch.
//! Nothing in this crate draws a screen.

#![forbid(unsafe_code)]

mod fraction;
mod sheet;

pub use fraction::Fraction;
pub use sheet::{
    Arrangement, Cell, CellId, Inside, NewCell, Placed, Refusal, Worksheet,
};
