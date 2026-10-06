//! The renderer: §7.2's four bind groups, the shape and curve organelles, and
//! the ID target. Every pixel is drawn from the tables through the vertex stage.

mod all_bytes;
mod apply;
mod band_probe;
mod bind_groups;
mod chart_extent;
mod draw;
mod draw_at;
mod frame;
mod frame_at;
mod layouts;
mod new;
mod picture;
mod picture_at;
mod pipelines;
mod read_target;
mod shader_source;
mod table_buffer;
mod target;
mod tick_bytes;
mod upload_all;

pub use band_probe::band_probe;

use joinn_visual::{
    BODY_ROW_BYTES, CELL_ROW_BYTES, CHART_ROW_BYTES, FRAME_ROW_BYTES, INCIDENCE_BYTES,
    LINK_ROW_BYTES, PORT_ROW_BYTES, STROKE_ROW_BYTES, STYLE_BYTES,
};

/// The ID target (§13.1): R body + 1, G cell + 1, B tag | n, A generation.
pub const ID_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Uint;
/// The color target of an offscreen render.
pub const OFFSCREEN_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Bytes per row of body, cell, port, link, incidence, style, chart, frame and
/// stroke, in that order.
pub(crate) const ROW_SIZES: [usize; 9] = [
    BODY_ROW_BYTES,
    CELL_ROW_BYTES,
    PORT_ROW_BYTES,
    LINK_ROW_BYTES,
    INCIDENCE_BYTES,
    STYLE_BYTES,
    CHART_ROW_BYTES,
    FRAME_ROW_BYTES,
    STROKE_ROW_BYTES,
];
/// The style table's index in `ROW_SIZES`. It is bound as a uniform: the
/// vertex stage has room for eight storage buffers, and the universe uses them.
pub(crate) const STYLE: usize = 5;
/// Bytes of the style uniform: sixteen styles, as four `vec4<u32>`.
pub(crate) const STYLE_UNIFORM_BYTES: usize = 64;
/// Bytes of the tick uniform: `level step pin_x pin_y ox oy fx fy width
/// height`, padded.
pub(crate) const TICK_BYTES: usize = 48;
/// Bytes of the pass uniform (Phase 6 draws one pass; it holds zeros).
pub(crate) const PASS_BYTES: usize = 16;
/// An organelle's instance index is `kind << KIND_SHIFT | slot`.
pub(crate) const KIND_SHIFT: u32 = 24;
/// Shape instances of the surface table.
pub(crate) const KIND_BODY: u32 = 0;
/// Shape instances of the cell table.
pub(crate) const KIND_CELL: u32 = 1;
/// Shape instances of the port table.
pub(crate) const KIND_PORT: u32 = 2;
/// Shape instances of the frame table.
pub(crate) const KIND_FRAME: u32 = 3;
/// Shape instances of the surface table drawn as a dot.
pub(crate) const KIND_DOT: u32 = 4;
/// Curve instances of the link table.
pub(crate) const KIND_LINK: u32 = 0;
/// Curve instances of the stroke table.
pub(crate) const KIND_STROKE: u32 = 1;

/// What one upload wrote.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Upload {
    /// Table rows written.
    pub rows: usize,
    /// Bytes written.
    pub bytes: usize,
}

/// Both targets of an offscreen render, read back row by row.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Picture {
    /// RGBA8, `width × height × 4` bytes.
    pub color: Vec<u8>,
    /// One ID texel per pixel.
    pub ids: Vec<[u32; 4]>,
}

/// Each organelle twice: the owner pass writes color and ID; the ghost pass
/// draws what is fading but not its owner band's, color only.
#[derive(Debug)]
pub(crate) struct Pipelines {
    shape: wgpu::RenderPipeline,
    shape_ghost: wgpu::RenderPipeline,
    curve: wgpu::RenderPipeline,
    curve_ghost: wgpu::RenderPipeline,
}

/// The pipelines, the table buffers as last uploaded, and the ID target.
#[derive(Debug)]
pub struct Renderer {
    format: wgpu::TextureFormat,
    layouts: [wgpu::BindGroupLayout; 4],
    pipelines: Pipelines,
    tick: wgpu::Buffer,
    pass: wgpu::Buffer,
    buffers: [wgpu::Buffer; 9],
    rows: [usize; 9],
    extent: i64,
    groups: [wgpu::BindGroup; 4],
    ids: Option<wgpu::Texture>,
    color: Option<wgpu::Texture>,
}

impl Renderer {
    /// The ID target of the last draw, if there was one.
    pub fn id_texture(&self) -> Option<&wgpu::Texture> {
        self.ids.as_ref()
    }

    /// The color format the pipelines write.
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
    }
}
