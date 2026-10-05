//! The tables of Part II §7.1, Phase 6 form. Every field is a `u32` or `i32`,
//! little-endian in field order, each row padded to a multiple of 16 bytes.
//! Tables hold layout units, never pixels.

mod empty;
mod free_slot;
mod put;
mod row_bytes;
mod shapes_of_tables;
mod table_bytes;

pub use row_bytes::row_bytes;
pub use shapes_of_tables::shapes_of_tables;
pub use table_bytes::table_bytes;

/// Flags bit 0 on every row: the slot draws.
pub const LIVE: u32 = 1;
/// Cell flags bit 1: the instance's last run refused at its membrane.
pub const REFUSED: u32 = 2;
/// Cell flags bit 2: a force's response whose out-port holds no value.
pub const LATENT: u32 = 4;
/// Port flags bit 1: the port holds a value.
pub const FILLED: u32 = 2;
/// Port flags bit 2: the port is on its body's surface, ∂(body), and draws
/// from the Summary band up. Written by universe scenes.
pub const SURFACE_PORT: u32 = 4;
/// Link kind of a wire.
pub const WIRE_KIND: u32 = 1;

/// Chart kinds, as the chart row's `kind` field holds them.
pub const CHART_UNIVERSE: u32 = 0;
/// A galaxy's chart.
pub const CHART_GALAXY: u32 = 1;
/// A system's chart.
pub const CHART_SYSTEM: u32 = 2;
/// A body's chart.
pub const CHART_BODY: u32 = 3;

/// Style ids (§2.8). Colours are decision, not truth.
pub const STYLE_BACKGROUND: u32 = 0;
/// Surface.
pub const STYLE_SURFACE: u32 = 1;
/// Cell.
pub const STYLE_CELL: u32 = 2;
/// Cell, refused.
pub const STYLE_CELL_REFUSED: u32 = 3;
/// Port, empty.
pub const STYLE_PORT_EMPTY: u32 = 4;
/// Port, filled.
pub const STYLE_PORT_FILLED: u32 = 5;
/// Wire.
pub const STYLE_WIRE: u32 = 6;
/// A response, latent. The shader picks refused, else latent, else the row's style.
pub const STYLE_RESPONSE_LATENT: u32 = 7;
/// An open system's or galaxy's frame (plan 7.2 §2.9).
pub const STYLE_FRAME: u32 = 8;
/// A lens node: a folded system or galaxy.
pub const STYLE_NODE: u32 = 9;
/// Text: labels, values and titles.
pub const STYLE_TEXT: u32 = 10;

/// One RGBA8 `u32` per style id: bytes R, G, B, A in little-endian order.
pub const STYLE_TABLE: [u32; 11] = [
    u32::from_le_bytes([0x15, 0x17, 0x1C, 0xFF]),
    u32::from_le_bytes([0x22, 0x26, 0x2E, 0xFF]),
    u32::from_le_bytes([0x2F, 0x5D, 0x8A, 0xFF]),
    u32::from_le_bytes([0xB0, 0x3A, 0x2E, 0xFF]),
    u32::from_le_bytes([0xC9, 0xCE, 0xD6, 0xFF]),
    u32::from_le_bytes([0xF2, 0xB1, 0x34, 0xFF]),
    u32::from_le_bytes([0x8A, 0x94, 0xA3, 0xFF]),
    u32::from_le_bytes([0x39, 0x41, 0x4D, 0xFF]),
    u32::from_le_bytes([0x1D, 0x21, 0x29, 0xFF]),
    u32::from_le_bytes([0x3B, 0x4C, 0x61, 0xFF]),
    u32::from_le_bytes([0xE8, 0xEA, 0xED, 0xFF]),
];

/// Bytes per body row: 7 fields, padded.
pub const BODY_ROW_BYTES: usize = 32;
/// Bytes per cell row: 9 fields, padded.
pub const CELL_ROW_BYTES: usize = 48;
/// Bytes per port row: 8 fields.
pub const PORT_ROW_BYTES: usize = 32;
/// Bytes per link row: 7 fields, padded.
pub const LINK_ROW_BYTES: usize = 32;
/// Bytes per incidence entry: one port slot.
pub const INCIDENCE_BYTES: usize = 4;
/// Bytes per style entry: one RGBA8.
pub const STYLE_BYTES: usize = 4;
/// Bytes per chart row: 7 fields, padded.
pub const CHART_ROW_BYTES: usize = 32;
/// Bytes per frame row: 7 fields, padded.
pub const FRAME_ROW_BYTES: usize = 32;
/// Bytes per stroke row: 12 fields.
pub const STROKE_ROW_BYTES: usize = 48;

/// Which table a row lives in.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Table {
    /// Group 1 binding 0.
    Body,
    /// Group 1 binding 1.
    Cell,
    /// Group 1 binding 2.
    Port,
    /// Group 1 binding 3.
    Link,
    /// Group 1 binding 4.
    Incidence,
    /// Group 1 binding 5.
    Chart,
    /// Group 1 binding 6.
    Frame,
    /// Group 1 binding 7.
    Stroke,
}

/// A chart (plan 7.2 §2.9). A body's chart slot is its body slot; a single-body
/// scene has one chart row, the body's, at origin 0.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ChartRow {
    /// Origin x relative to the anchor's origin, layout units.
    pub origin_x: i32,
    /// Origin y relative to the anchor's origin.
    pub origin_y: i32,
    /// Parent chart slot; the root is its own parent.
    pub parent: u32,
    /// `CHART_UNIVERSE`, `CHART_GALAXY`, `CHART_SYSTEM` or `CHART_BODY`.
    pub kind: u32,
    /// `max(w, h)`, layout units.
    pub size: u32,
    /// Slot generation, from 1.
    pub generation: u32,
    /// Bit 0 live.
    pub flags: u32,
}

/// A system's or galaxy's frame: open, it is drawn with its title and
/// children; folded, it is a lens node that owns its pixels.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct FrameRow {
    /// Its chart slot.
    pub chart: u32,
    /// Width, layout units.
    pub w: u32,
    /// Height.
    pub h: u32,
    /// `CHART_GALAXY` or `CHART_SYSTEM`.
    pub kind: u32,
    /// Galaxy or system number in lens order.
    pub index: u32,
    /// Slot generation, from 1.
    pub generation: u32,
    /// Bit 0 live.
    pub flags: u32,
}

/// One stroke of text, a capsule in sixteenths of its chart.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StrokeRow {
    /// Chart slot.
    pub chart: u32,
    /// Start x, sixteenths.
    pub x0: i32,
    /// Start y.
    pub y0: i32,
    /// End x.
    pub x1: i32,
    /// End y.
    pub y1: i32,
    /// Half-width, sixteenths.
    pub half_width: u32,
    /// `[R, G, B]` of the owner's ID.
    pub owner: [u32; 3],
    /// Style id.
    pub style: u32,
    /// The owner's generation.
    pub generation: u32,
    /// Bit 0 live.
    pub flags: u32,
}

/// A surface.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BodyRow {
    /// Surface left.
    pub x: i32,
    /// Surface top.
    pub y: i32,
    /// Surface width.
    pub w: i32,
    /// Surface height.
    pub h: i32,
    /// Corner radius.
    pub radius: u32,
    /// Slot generation, from 1.
    pub generation: u32,
    /// Bit 0 live.
    pub flags: u32,
}

/// An instance's cell.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct CellRow {
    /// Body slot.
    pub body: u32,
    /// Left.
    pub x: i32,
    /// Top.
    pub y: i32,
    /// Width.
    pub w: i32,
    /// Height.
    pub h: i32,
    /// Corner radius.
    pub radius: u32,
    /// Slot generation, from 1.
    pub generation: u32,
    /// Style id.
    pub style: u32,
    /// Bit 0 live, bit 1 refused, bit 2 latent.
    pub flags: u32,
}

/// A port.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PortRow {
    /// Cell slot.
    pub cell: u32,
    /// Position.
    pub position: u32,
    /// 0 in, 1 out.
    pub direction: u32,
    /// Centre x.
    pub x: i32,
    /// Centre y.
    pub y: i32,
    /// Radius.
    pub radius: u32,
    /// Slot generation, from 1.
    pub generation: u32,
    /// Bit 0 live, bit 1 filled.
    pub flags: u32,
}

/// A link. Wire `s` owns incidence entries `2s` (source) and `2s + 1` (destination).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LinkRow {
    /// 1: wire.
    pub kind: u32,
    /// First incidence entry.
    pub start: u32,
    /// Incidence entries.
    pub count: u32,
    /// Body slot.
    pub body: u32,
    /// Half-width in quarters of a layout unit.
    pub half_width_quarters: u32,
    /// Slot generation, from 1.
    pub generation: u32,
    /// Bit 0 live.
    pub flags: u32,
}

/// Every table the GPU reads.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Tables {
    /// Group 1 binding 0.
    pub body: Vec<BodyRow>,
    /// Group 1 binding 1.
    pub cell: Vec<CellRow>,
    /// Group 1 binding 2.
    pub port: Vec<PortRow>,
    /// Group 1 binding 3.
    pub link: Vec<LinkRow>,
    /// Group 1 binding 4: port slots.
    pub incidence: Vec<u32>,
    /// Group 2 binding 0.
    pub style: Vec<u32>,
    /// Group 1 binding 5.
    pub chart: Vec<ChartRow>,
    /// Group 1 binding 6.
    pub frame: Vec<FrameRow>,
    /// Group 1 binding 7.
    pub stroke: Vec<StrokeRow>,
}

/// One row to write.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Row {
    Body(BodyRow),
    Cell(CellRow),
    Port(PortRow),
    Link(LinkRow),
    Incidence(u32),
    Chart(ChartRow),
    Frame(FrameRow),
    Stroke(StrokeRow),
}

/// The tables as the GPU uploads them and as regrow compares them.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TableBytes {
    /// Body rows.
    pub body: Vec<u8>,
    /// Cell rows.
    pub cell: Vec<u8>,
    /// Port rows.
    pub port: Vec<u8>,
    /// Link rows.
    pub link: Vec<u8>,
    /// Incidence entries.
    pub incidence: Vec<u8>,
    /// Style entries.
    pub style: Vec<u8>,
    /// Chart rows.
    pub chart: Vec<u8>,
    /// Frame rows.
    pub frame: Vec<u8>,
    /// Stroke rows.
    pub stroke: Vec<u8>,
}

/// A changed row: `(table, slot)`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct RowWrite {
    /// The table.
    pub table: Table,
    /// The slot, or the incidence entry.
    pub slot: u32,
}

/// Rows that changed, each once, in `(table, slot)` order.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Delta {
    /// The rows.
    pub rows: Vec<RowWrite>,
}
