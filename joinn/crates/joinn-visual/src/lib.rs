//! The upward wrap: layout, camera, tables and exact picking. No float, no GPU.

#![forbid(unsafe_code)]

mod camera;
#[cfg(test)]
mod dropped;
#[cfg(test)]
mod fixtures;
mod layout;
mod pick;
mod refuse;
mod scene;
#[cfg(test)]
mod script;
mod tables;

pub use camera::{
    Camera, ChartId, FOCUS_UNIT, FitCamera, LEVEL_MAX, LEVEL_MIN, NOTCH, PIXEL_UNIT,
    STANDARD_VIEWPORTS, STEPS, Zoom, fit, print_camera, print_zoom,
};
pub use layout::{
    CELL_RADIUS, CELL_W, CellBox, GAP_X, GAP_Y, Layout, MARGIN, PITCH, PORT_INSET, PORT_RADIUS,
    PortDot, Rect, SURFACE_RADIUS, WIRE_HALF_WIDTH_QUARTERS, WireSeg, layout, layout_contact,
    print_layout,
};
pub use pick::{
    Class, Geom, Owner, PORT_TAG, Pick, PickImage, Shape, TAG_MASK, WIRE_TAG, cpu_pick,
    cpu_pick_reference, owner_of_layout, print_owner, shapes_of_layout,
};
pub use scene::Scene;
pub use tables::{
    BODY_ROW_BYTES, BodyRow, CELL_ROW_BYTES, CellRow, Delta, FILLED, INCIDENCE_BYTES, LATENT,
    LINK_ROW_BYTES, LIVE, LinkRow, PORT_ROW_BYTES, PortRow, REFUSED, RowWrite, STYLE_BACKGROUND,
    STYLE_BYTES, STYLE_CELL, STYLE_CELL_REFUSED, STYLE_PORT_EMPTY, STYLE_PORT_FILLED,
    STYLE_RESPONSE_LATENT, STYLE_SURFACE, STYLE_TABLE, STYLE_WIRE, Table, TableBytes, Tables,
    WIRE_KIND, row_bytes, shapes_of_tables, table_bytes,
};
