//! The renderer: §7.2's four bind groups, the shape and curve organelles, and
//! the ID target. Every pixel is drawn from the tables through the vertex stage.

mod apply;
mod bind_groups;
mod draw;
mod frame;
mod layouts;
mod new;
mod picture;
mod pipelines;
mod read_target;
mod shader_source;
mod table_buffer;
mod target;
mod tick_bytes;
mod upload_all;

use joinn_visual::{
    BODY_ROW_BYTES, CELL_ROW_BYTES, INCIDENCE_BYTES, LINK_ROW_BYTES, PORT_ROW_BYTES, STYLE_BYTES,
};

/// The ID target (§13.1): R body + 1, G cell + 1, B tag | n, A generation.
pub const ID_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba32Uint;
/// The color target of an offscreen render.
pub const OFFSCREEN_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

/// Bytes per row of body, cell, port, link, incidence and style, in that order.
pub(crate) const ROW_SIZES: [usize; 6] = [
    BODY_ROW_BYTES,
    CELL_ROW_BYTES,
    PORT_ROW_BYTES,
    LINK_ROW_BYTES,
    INCIDENCE_BYTES,
    STYLE_BYTES,
];
/// Bytes of the tick uniform: `k ox oy width height`, padded.
pub(crate) const TICK_BYTES: usize = 32;
/// Bytes of the pass uniform (Phase 6 draws one pass; it holds zeros).
pub(crate) const PASS_BYTES: usize = 16;
/// The shape organelle's instance index is `kind << KIND_SHIFT | slot`.
pub(crate) const KIND_SHIFT: u32 = 24;
/// Shape instances of the surface table.
pub(crate) const KIND_BODY: u32 = 0;
/// Shape instances of the cell table.
pub(crate) const KIND_CELL: u32 = 1;
/// Shape instances of the port table.
pub(crate) const KIND_PORT: u32 = 2;

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

/// The pipelines, the table buffers as last uploaded, and the ID target.
#[derive(Debug)]
pub struct Renderer {
    format: wgpu::TextureFormat,
    layouts: [wgpu::BindGroupLayout; 4],
    shape: wgpu::RenderPipeline,
    curve: wgpu::RenderPipeline,
    tick: wgpu::Buffer,
    pass: wgpu::Buffer,
    buffers: [wgpu::Buffer; 6],
    rows: [usize; 6],
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
