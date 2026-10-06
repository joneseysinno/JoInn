//! The upward wrap: layout, camera, tables and exact picking. No float, no GPU.

#![forbid(unsafe_code)]

mod bands;
mod camera;
mod charts;
#[cfg(test)]
mod dropped;
#[cfg(test)]
mod fixtures;
mod font;
mod layout;
mod lens_cut;
mod pick;
mod refuse;
mod routes;
mod scene;
#[cfg(test)]
mod script;
mod tables;
mod universe_scene;

pub use bands::{Band, THRESHOLDS, fade_window, owner_band};
pub use camera::{
    Camera, ChartId, FOCUS_UNIT, FitCamera, LEVEL_MAX, LEVEL_MIN, NOTCH, PIXEL_UNIT,
    STANDARD_VIEWPORTS, STEPS, Zoom, fit, print_camera, print_zoom,
};
pub use charts::{
    BODIES_MAX, BODY_MAX, Chart, ChartKind, GALAXIES_MAX, GALAXY_SIZE, SYSTEM_SIZE, SYSTEMS_MAX,
    UNIVERSE_SIZE, UniverseLayout, layout_universe,
};
pub use font::{
    ADVANCE, GALAXY_TITLE_SIZE, GRID_DESCENT, GRID_H, GRID_W, GlyphStroke, LABEL_SIZE, STROKE_SET,
    SYSTEM_TITLE_SIZE, TextSize, TextStroke, TitleOf, VALUE_GLYPHS, VALUE_SIZE, cell_label, glyph,
    port_value, text_strokes, title, value_text,
};
pub use layout::{
    CELL_RADIUS, CELL_W, CellBox, GAP_X, GAP_Y, Layout, MARGIN, PITCH, PORT_INSET, PORT_RADIUS,
    PortDot, Rect, SURFACE_RADIUS, WIRE_HALF_WIDTH_QUARTERS, WireSeg, layout, layout_contact,
    print_layout,
};
pub use lens_cut::{Cut, CutEntry, CutForm, cut, touches};
pub use pick::{
    Class, GALAXY_TAG, Geom, Owner, PORT_TAG, Pick, PickImage, SYSTEM_TAG, Shape, TAG_MASK,
    WIRE_TAG, cpu_pick, cpu_pick_at, cpu_pick_reference, cpu_pick_reference_at, cpu_pick_sample,
    owner_of_layout, print_owner, shapes_of_layout,
};
pub use routes::{
    Fold, GALAXY_GUTTER_X, GALAXY_GUTTER_Y, GALAXY_REACH, GALAXY_SIDES, Graph, Line, Piece,
    PieceKind, Route, Routes, SIXTEENTHS, SYSTEM_GUTTER_X, SYSTEM_GUTTER_Y, SYSTEM_REACH,
    SYSTEM_SIDES, Touch, UNIVERSE_GUTTER_X, UNIVERSE_GUTTER_Y, crossings, grid_lines, routes,
    routing_graph, side,
};
pub use scene::Scene;
pub use tables::{
    BODY_ROW_BYTES, BodyRow, CELL_ROW_BYTES, CHART_BODY, CHART_GALAXY, CHART_ROW_BYTES,
    CHART_SYSTEM, CHART_UNIVERSE, CellRow, ChartRow, Delta, FILLED, FRAME_ROW_BYTES, FrameRow,
    INCIDENCE_BYTES, LATENT, LINK_ROW_BYTES, LIVE, LinkRow, PORT_ROW_BYTES, PortRow, REFUSED,
    RowWrite, STROKE_ROW_BYTES, STYLE_BACKGROUND, STYLE_BYTES, STYLE_CELL, STYLE_CELL_REFUSED,
    STYLE_FRAME, STYLE_NODE, STYLE_PORT_EMPTY, STYLE_PORT_FILLED, STYLE_RESPONSE_LATENT,
    STYLE_SURFACE, STYLE_TABLE, STYLE_TEXT, STYLE_WIRE, SURFACE_PORT, StrokeRow, Table, TableBytes,
    Tables, WIRE_KIND, row_bytes, shapes_of_tables, table_bytes,
};
pub use universe_scene::UniverseScene;
