//! The upward wrap: layout, camera, tables and exact picking. No float, no GPU.

#![forbid(unsafe_code)]

mod camera;
#[cfg(test)]
mod fixtures;
mod layout;
mod refuse;

pub use camera::{Camera, STANDARD_VIEWPORTS, fit, print_camera};
pub use layout::{
    CELL_RADIUS, CELL_W, CellBox, GAP_X, GAP_Y, Layout, MARGIN, MEMBRANE_RADIUS, PITCH, PORT_INSET,
    PORT_RADIUS, PortDot, Rect, WIRE_HALF_WIDTH_QUARTERS, WireSeg, layout, print_layout,
};
